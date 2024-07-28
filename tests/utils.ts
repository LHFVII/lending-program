import { BanksClient, BanksTransactionMeta, ProgramTestContext } from 'solana-bankrun';
import { SystemProgram, PublicKey, Keypair, Transaction } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as token from "@solana/spl-token";


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
    console.log('gotten public keys')
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