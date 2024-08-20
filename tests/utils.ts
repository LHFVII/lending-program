import { BanksClient, BanksTransactionMeta } from 'solana-bankrun';
import { SystemProgram, PublicKey, Keypair, Transaction, LAMPORTS_PER_SOL} from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as token from "@solana/spl-token";
import { AnchorProvider, Program } from '@coral-xyz/anchor';
import { assert } from 'chai';
import buffer from 'buffer';


import {
  BankrunConnection,
  BankrunContextWrapper,
} from './bankrunConnection';
import pythIDL from '../target/idl/pyth.json';

const empty32Buffer = buffer.Buffer.alloc(32);


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
    price: number = 5 * 10e7,
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

    const priceFeedAddress = await createPriceFeedBankrun({
      oracleProgram: program,
      context: context,
      initPrice: price,
      expo: expo,
      confidence,
    });
  
    const feedData = await getFeedDataNoProgram(
      context.connection,
      priceFeedAddress
    );
    if (feedData.price !== price) {
      console.log('mockOracle precision error:', feedData.price, '!=', price);
    }
    //assert.ok(Math.abs(feedData.price - price) < 1e-10);
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
  tx.feePayer = context.context.payer.publicKey;
  tx.recentBlockhash = context.context.lastBlockhash;
  tx.sign(...[collateralTokenFeed, context.context.payer]);
  await context.connection.sendTransaction(tx);
  return collateralTokenFeed.publicKey;
  };

  export const getFeedDataNoProgram = async (
    connection: BankrunConnection,
    priceFeed: PublicKey
  ) => {
    const info = await connection.getAccountInfoAndContext(priceFeed);
    return parsePriceData(info.value.data);
  };

  const parsePriceData = (data) => {
    // Pyth magic number.
    const magic = data.readUInt32LE(0);
    // Program version.
    const version = data.readUInt32LE(4);
    // Account type.
    const type = data.readUInt32LE(8);
    // Price account size.
    const size = data.readUInt32LE(12);
    // Price or calculation type.
    const priceType = data.readUInt32LE(16);
    // Price exponent.
    const exponent = data.readInt32LE(20);
    // Number of component prices.
    const numComponentPrices = data.readUInt32LE(24);
    // unused
    // const unused = accountInfo.data.readUInt32LE(28)
    // Currently accumulating price slot.
    const currentSlot = readBigUInt64LE(data, 32);
    // Valid on-chain slot of aggregate price.
    const validSlot = readBigUInt64LE(data, 40);
    // Time-weighted average price.
    const twapComponent = readBigInt64LE(data, 48);
    const twap = Number(twapComponent) * 10 ** exponent;
    // Annualized price volatility.
    const avolComponent = readBigUInt64LE(data, 56);
    const avol = Number(avolComponent) * 10 ** exponent;
    // Space for future derived values.
    const drv0Component = readBigInt64LE(data, 64);
    const drv0 = Number(drv0Component) * 10 ** exponent;
    const drv1Component = readBigInt64LE(data, 72);
    const drv1 = Number(drv1Component) * 10 ** exponent;
    const drv2Component = readBigInt64LE(data, 80);
    const drv2 = Number(drv2Component) * 10 ** exponent;
    const drv3Component = readBigInt64LE(data, 88);
    const drv3 = Number(drv3Component) * 10 ** exponent;
    const drv4Component = readBigInt64LE(data, 96);
    const drv4 = Number(drv4Component) * 10 ** exponent;
    const drv5Component = readBigInt64LE(data, 104);
    const drv5 = Number(drv5Component) * 10 ** exponent;
    // Product id / reference account.
    const productAccountKey = new anchor.web3.PublicKey(data.slice(112, 144));
    // Next price account in list.
    const nextPriceAccountKey = PKorNull(data.slice(144, 176));
    // Aggregate price updater.
    const aggregatePriceUpdaterAccountKey = new anchor.web3.PublicKey(
      data.slice(176, 208)
    );
    const aggregatePriceInfo = parsePriceInfo(data.slice(208, 240), exponent);
    // Price components - up to 32.
    const priceComponents = [];
    let offset = 240;
    let shouldContinue = true;
    while (offset < data.length && shouldContinue) {
      const publisher = PKorNull(data.slice(offset, offset + 32));
      offset += 32;
      if (publisher) {
        const aggregate = parsePriceInfo(
          data.slice(offset, offset + 32),
          exponent
        );
        offset += 32;
        const latest = parsePriceInfo(data.slice(offset, offset + 32), exponent);
        offset += 32;
        priceComponents.push({ publisher, aggregate, latest });
      } else {
        shouldContinue = false;
      }
    }
    return Object.assign(
      Object.assign(
        {
          magic,
          version,
          type,
          size,
          priceType,
          exponent,
          numComponentPrices,
          currentSlot,
          validSlot,
          twapComponent,
          twap,
          avolComponent,
          avol,
          drv0Component,
          drv0,
          drv1Component,
          drv1,
          drv2Component,
          drv2,
          drv3Component,
          drv3,
          drv4Component,
          drv4,
          drv5Component,
          drv5,
          productAccountKey,
          nextPriceAccountKey,
          aggregatePriceUpdaterAccountKey,
        },
        aggregatePriceInfo
      ),
      { priceComponents }
    );
  };

  function readBigUInt64LE(buffer, offset = 0) {
    validateNumber(offset, 'offset');
    const first = buffer[offset];
    const last = buffer[offset + 7];
    if (first === undefined || last === undefined)
      boundsError(offset, buffer.length - 8);
    const lo =
      first +
      buffer[++offset] * 2 ** 8 +
      buffer[++offset] * 2 ** 16 +
      buffer[++offset] * 2 ** 24;
    const hi =
      buffer[++offset] +
      buffer[++offset] * 2 ** 8 +
      buffer[++offset] * 2 ** 16 +
      last * 2 ** 24;
    return BigInt(lo) + (BigInt(hi) << BigInt(32)); // tslint:disable-line:no-bitwise
  }

  function validateNumber(value, name) {
    if (typeof value !== 'number')
      throw ERR_INVALID_ARG_TYPE(name, 'number', value);
  }
  // https://github.com/nodejs/node/blob/v14.17.0/lib/internal/buffer.js#L68-L80
  function boundsError(value, length) {
    if (Math.floor(value) !== value) {
      validateNumber(value, 'offset');
      throw ERR_OUT_OF_RANGE('offset', 'an integer', value);
    }
    if (length < 0) throw ERR_BUFFER_OUT_OF_BOUNDS();
    throw ERR_OUT_OF_RANGE('offset', `>= 0 and <= ${length}`, value);
  }

  
const ERR_BUFFER_OUT_OF_BOUNDS = () =>
	new Error('Attempt to access memory outside buffer bounds');

const ERR_INVALID_ARG_TYPE = (name, expected, actual) =>
	new Error(
		`The "${name}" argument must be of type ${expected}. Received ${actual}`
	);

const ERR_OUT_OF_RANGE = (str, range, received) =>
	new Error(
		`The value of "${str} is out of range. It must be ${range}. Received ${received}`
	);

  function readBigInt64LE(buffer, offset = 0) {
    validateNumber(offset, 'offset');
    const first = buffer[offset];
    const last = buffer[offset + 7];
    if (first === undefined || last === undefined)
      boundsError(offset, buffer.length - 8);
    const val =
      buffer[offset + 4] +
      buffer[offset + 5] * 2 ** 8 +
      buffer[offset + 6] * 2 ** 16 +
      (last << 24); // Overflow
    return (
      (BigInt(val) << BigInt(32)) +
      BigInt(
        first +
          buffer[++offset] * 2 ** 8 +
          buffer[++offset] * 2 ** 16 +
          buffer[++offset] * 2 ** 24
      )
    );
  }  
  const PKorNull = (data) =>
    data.equals(empty32Buffer) ? null : new anchor.web3.PublicKey(data);
  
  export const createPriceFeed = async ({
    oracleProgram,
    initPrice,
    confidence = undefined,
    expo = -4,
  }: {
    oracleProgram: Program;
    initPrice: number;
    confidence?: number;
    expo?: number;
  }): Promise<PublicKey> => {
    const conf = new anchor.BN(confidence) || new anchor.BN((initPrice / 10) * 10 ** -expo);
    const collateralTokenFeed = new anchor.web3.Account();
    const txid = await oracleProgram.rpc.initialize(
      new anchor.BN(initPrice * 10 ** -expo),
      expo,
      conf,
      {
        accounts: { price: collateralTokenFeed.publicKey },
        signers: [collateralTokenFeed],
        instructions: [
          anchor.web3.SystemProgram.createAccount({
            // @ts-ignore
            fromPubkey: oracleProgram.provider.wallet.publicKey,
            newAccountPubkey: collateralTokenFeed.publicKey,
            space: 134,
            lamports:
              await oracleProgram.provider.connection.getMinimumBalanceForRentExemption(
                134
              ),
            programId: oracleProgram.programId,
          }),
        ],
      }
    );
    return collateralTokenFeed.publicKey;
  };

 
  const parsePriceInfo = (data, exponent) => {
    // Aggregate price.
    const priceComponent = data.readBigUInt64LE(0);
    const price = Number(priceComponent) * 10 ** exponent;
    // Aggregate confidence.
    const confidenceComponent = data.readBigUInt64LE(8);
    const confidence = Number(confidenceComponent) * 10 ** exponent;
    // Aggregate status.
    const status = data.readUInt32LE(16);
    // Aggregate corporate action.
    const corporateAction = data.readUInt32LE(20);
    // Aggregate publish slot.
    const publishSlot = data.readBigUInt64LE(24);
    return {
      priceComponent,
      price,
      confidenceComponent,
      confidence,
      status,
      corporateAction,
      publishSlot,
    };
  };

  