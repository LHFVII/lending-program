import { BanksClient, BanksTransactionMeta } from 'solana-bankrun';
import { SystemProgram, PublicKey, Keypair, Transaction, LAMPORTS_PER_SOL} from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as token from "@solana/spl-token";
import { AnchorProvider, Program } from '@coral-xyz/anchor';

import { assert } from 'chai';

import {
  BankrunContextWrapper,
} from './bankrunConnection';
import pythIDL from '../target/idl/pyth.json';



export async function createMint(
    banksClient: BanksClient,
    payer: Keypair,
    mintAuthority: PublicKey,
    freezeAuthority: PublicKey | null,
    decimals: number,
    keypair = Keypair.generate(),
    programId = token.TOKEN_PROGRAM_ID
  ): Promise<PublicKey> {
    let rent = await banksClient.getRent();
  
    const tx = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: keypair.publicKey,
        space: token.MINT_SIZE,
        lamports: Number(await rent.minimumBalance(BigInt(token.MINT_SIZE))),
        programId: token.TOKEN_PROGRAM_ID,
      }),
      token.createInitializeMint2Instruction(
        keypair.publicKey,
        decimals,
        mintAuthority,
        freezeAuthority,
        programId
      ),
      
    );
    [tx.recentBlockhash] = (await banksClient.getLatestBlockhash())!;
    tx.sign(payer, keypair);
  
    await banksClient.processTransaction(tx);

  
    return keypair.publicKey;
  }

  export async function mintTo(
    banksClient: BanksClient,
    payer: anchor.web3.Signer,
    mint: PublicKey,
    destination: PublicKey,
    authority: anchor.web3.Signer | PublicKey,
    amount: number | bigint,
    multiSigners: anchor.web3.Signer[] = [],
    programId = token.TOKEN_PROGRAM_ID
  ): Promise<BanksTransactionMeta> {
    const [authorityPublicKey, signers] = getSigners(authority, multiSigners);
    const tx = new Transaction().add(
      token.createMintToInstruction(
        mint,
        destination,
        authorityPublicKey,
        amount,
        multiSigners,
        programId
      )
    );
    [tx.recentBlockhash] = await banksClient.getLatestBlockhash();
    tx.sign(payer, ...signers);
  
    return await banksClient.processTransaction(tx);
  }
  
  export function getSigners(
    signerOrMultisig: anchor.web3.Signer | PublicKey,
    multiSigners: anchor.web3.Signer[]
  ): [PublicKey, anchor.web3.Signer[]] {
    return signerOrMultisig instanceof PublicKey
      ? [signerOrMultisig, multiSigners]
      : [signerOrMultisig.publicKey, [signerOrMultisig]];
  }

  export async function createAssociatedTokenAccount(
    banksClient: BanksClient,
    payer: anchor.web3.Signer,
    mint: PublicKey,
    owner: PublicKey,
    programId = token.TOKEN_PROGRAM_ID,
    associatedTokenProgramId = token.ASSOCIATED_TOKEN_PROGRAM_ID
  ): Promise<PublicKey> {
    const associatedToken = token.getAssociatedTokenAddressSync(
      mint,
      owner,
      false,
      programId,
      associatedTokenProgramId
    );
  
    const tx = new Transaction().add(
      token.createAssociatedTokenAccountInstruction(
        payer.publicKey,
        associatedToken,
        owner,
        mint,
        programId,
        associatedTokenProgramId
      )
    );
  
    [tx.recentBlockhash] = await banksClient.getLatestBlockhash();
    tx.sign(payer);
  
    await banksClient.processTransaction(tx);
  
    return associatedToken;
  }

  export async function mockOracleNoProgram(
    context: BankrunContextWrapper,
    price: number = 50 * 10e7,
    expo = -7,
    confidence?: number
  ): Promise<PublicKey> {
    const provider = new AnchorProvider(
      context.connection.toConnection(),
      context.provider.wallet,
      {
        commitment: 'processed',
      }
    );
  
    const program = new Program(
      pythIDL as anchor.Idl,
      provider
    );

    console.log(program.programId.toString());
  
    const priceFeedAddress = await createPriceFeedBankrun({
      oracleProgram: program,
      context: context,
      initPrice: price,
      expo: expo,
      confidence,
    });
    console.log('mockOracleNoProgram:', priceFeedAddress.toString());
    // @ts-ignore
    const feedData = await getFeedDataNoProgram(
      context.connection,
      priceFeedAddress
    );
    if (feedData.price !== price) {
      console.log('mockOracle precision error:', feedData.price, '!=', price);
    }
    assert.ok(Math.abs(feedData.price - price) < 1e-10);
  
    return priceFeedAddress;
  }

  
  export const createPriceFeedBankrun = async ({
    oracleProgram,
    context,
    initPrice,
    confidence = undefined,
    expo = -4,
  }: {
    oracleProgram: Program;
    context: BankrunContextWrapper;
    initPrice: number;
    confidence?: number;
    expo?: number;
  }): Promise<PublicKey> => {
    const conf = confidence ? new anchor.BN(confidence) : new anchor.BN((initPrice / 10) * 10 ** -expo);
  const collateralTokenFeed = new anchor.web3.Account();
  const createAccountIx = anchor.web3.SystemProgram.createAccount({
    fromPubkey: context.context.payer.publicKey,
    newAccountPubkey: collateralTokenFeed.publicKey,
    space: 3312,
    lamports: LAMPORTS_PER_SOL / 20, // just hardcode based on mainnet
    programId: oracleProgram.programId,
  });

  const price = new anchor.BN(initPrice * 10 ** -expo);

  // Use methods instead of instruction
  const ix = await oracleProgram.methods
    .initialize(price, expo, conf)
    .accounts({
      price: collateralTokenFeed.publicKey,
    })
    .instruction();

  const tx = new Transaction().add(createAccountIx).add(ix);
  console.log('surubum')
  tx.feePayer = context.context.payer.publicKey;
  tx.recentBlockhash = context.context.lastBlockhash;
  tx.sign(...[collateralTokenFeed, context.context.payer]);
  console.log('surubum 2')
  console.log(context)
  await context.connection.sendTransaction(tx);
  console.log('surubum 3')
  return collateralTokenFeed.publicKey;
  };

 
