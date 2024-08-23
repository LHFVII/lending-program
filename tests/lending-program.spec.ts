import { startAnchor} from 'solana-bankrun';
import { BankrunProvider } from 'anchor-bankrun';
import { PublicKey, Keypair } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { LendingProgram } from "../target/types/lending_program";
import { expect } from 'chai';
import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, unpackAccount } from '@solana/spl-token';
import { createAssociatedTokenAccount, createMint, mintTo } from './utils';
import { BankrunContextWrapper } from './bankrunConnection';
import { PythSolanaReceiver } from '@pythnetwork/pyth-solana-receiver';
 
const IDL = require("../target/idl/lending_program.json");

const transferAmount = 1_000_000_000;
const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: PublicKey = new PublicKey(
    'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
  );


describe("Create a system account", async () => {
    let context;
    let userOne;
    let puppetProgram;
    let provider;
    let banksClient;
    let mintAuthority;
    let payer;
    let USDC;
    let SOL;
    let userUsdcAccount;
    let userTokenAddress;
    
    let poolUsdcAssociatedTokenAddress;
    let poolSOLAssociatedTokenAddress;
    let pythSolanaReceiver
    let bankrunContextWrapper: BankrunContextWrapper;
    let connection
    let solUsdPriceFeedAccount


    before(async () => {
        const programId = new PublicKey('77B3AdNp6RRzsVMQSWAUVv28RXmA8YJVAfWkimRTwXi6')
        userOne = Keypair.generate();
        context = await startAnchor("",[{name:"lending_program", programId: programId}],[])
        provider = new BankrunProvider(context);
        bankrunContextWrapper = new BankrunContextWrapper(context);
        connection = bankrunContextWrapper.connection.toConnection();
        pythSolanaReceiver = new PythSolanaReceiver({
            connection,
            wallet: provider.wallet,
        });
      
        const SOL_PRICE_FEED_ID =
        '0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d';
        pythSolanaReceiver
        solUsdPriceFeedAccount = pythSolanaReceiver
        .getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID)
        .toBase58();

        puppetProgram = new Program<LendingProgram>(IDL, provider);
        banksClient = context.banksClient;
        mintAuthority = anchor.web3.Keypair.generate();
        payer = provider.wallet.payer;
        const transferTransaction = new anchor.web3.Transaction().add(
            anchor.web3.SystemProgram.transfer({
            fromPubkey: puppetProgram.provider.publicKey,
            toPubkey: userOne.publicKey,
            lamports: transferAmount,
            })
        );
        await provider.sendAndConfirm(transferTransaction, [provider.wallet.payer]);
        USDC = await createMint(
            banksClient,
            payer,
            payer.publicKey,
            payer.publicKey,
            6
          );
        SOL = await createMint(
            banksClient,
            payer,
            payer.publicKey,
            null,
            2
          );
        userUsdcAccount = await createAssociatedTokenAccount(
            banksClient,
            userOne,
            USDC,
            userOne.publicKey
        );

        await puppetProgram.methods.initializeUser()
            .accounts({payer: userOne.publicKey})
            .signers([userOne])
            .rpc();
        
        await mintTo(
            banksClient,
            userOne,
            USDC,
            userUsdcAccount,
            payer,
            1_000_000 * 10 ** 6,
        );
        
        await puppetProgram.methods.initializePool(new anchor.BN(10),new anchor.BN(100000))
            .accounts({payer: puppetProgram.provider.publicKey, mint: USDC, tokenProgram: TOKEN_PROGRAM_ID})
            .rpc();
        
        await puppetProgram.methods.initializePool(new anchor.BN(10),new anchor.BN(100000))
            .accounts({payer: puppetProgram.provider.publicKey, mint: SOL, tokenProgram: TOKEN_PROGRAM_ID})
            .rpc();
        
        userTokenAddress = await getAssociatedTokenAddressSync(USDC, userOne.publicKey);
        //userSecondMintTokenAddress = await getAssociatedTokenAddressSync(secondMint, userOne.publicKey);
        [poolUsdcAssociatedTokenAddress] = await PublicKey.findProgramAddressSync([
            Buffer.from('treasury'),
            USDC.toBuffer(),
        ],puppetProgram.programId);

        [poolSOLAssociatedTokenAddress] = await PublicKey.findProgramAddressSync([
            Buffer.from('treasury'),
            SOL.toBuffer(),
        ],puppetProgram.programId);
    });

    it("Deposit collateral", async () => {
        let userTokenAccount = await banksClient.getAccount(userTokenAddress);
        let unpackedAccount = unpackAccount(userTokenAddress, userTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedAccount.amount).to.equal(BigInt(1_000_000 * 10 ** 6));

        let PoolTokenAccount = await banksClient.getAccount(poolUsdcAssociatedTokenAddress);
        let unpackedPoolAccount = unpackAccount(poolUsdcAssociatedTokenAddress, PoolTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedPoolAccount.amount).to.equal(BigInt(0));

        const [userAddress] = PublicKey.findProgramAddressSync([userOne.publicKey.toBuffer()], puppetProgram.programId);

        await puppetProgram.methods.depositCollateral(new anchor.BN(100))
            .accounts({payer: userOne.publicKey, mint: USDC, userAccount: userAddress, poolTokenAccount: poolUsdcAssociatedTokenAddress, tokenProgram: TOKEN_PROGRAM_ID})
            .signers([userOne])
            .rpc();
        
        // Check pool and user USDC balance
        PoolTokenAccount = await banksClient.getAccount(poolUsdcAssociatedTokenAddress);
        unpackedPoolAccount = unpackAccount(poolUsdcAssociatedTokenAddress, PoolTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedPoolAccount.amount).to.equal(BigInt(100));
        userTokenAccount = await banksClient.getAccount(userTokenAddress);
        unpackedAccount = unpackAccount(userTokenAddress, userTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedAccount.amount).to.equal(BigInt((1_000_000 * 10 ** 6)-100));
    });

    /*it("Withdraw", async () => {
        const bankrunContextWrapper = new BankrunContextWrapper(context);
        const priceFeedAddress = await mockOracleNoProgram(bankrunContextWrapper,150);
        const [poolAssociatedTokenAddress] = await PublicKey.findProgramAddressSync([
            provider.publicKey.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            USDC.toBuffer(),
        ],SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID)
        let PoolTokenAccount = await banksClient.getAccount(poolAssociatedTokenAddress);
        let unpackedPoolAccount = unpackAccount(poolAssociatedTokenAddress, PoolTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedPoolAccount.amount).to.equal(BigInt(100));
        
        const [userAddress] = PublicKey.findProgramAddressSync([userOne.publicKey.toBuffer()], puppetProgram.programId);
        
        await puppetProgram.methods.withdrawCollateral(new anchor.BN(30))
            .accounts({payer: payer, collateralMint: USDC, userAccount: userAddress, poolTokenAccount: poolAssociatedTokenAddress, userTokenAccount: userTokenAddress, priceFeed: priceFeedAddress})
            .signers([payer])
            .rpc();
        
        PoolTokenAccount = await banksClient.getAccount(poolAssociatedTokenAddress);
        unpackedPoolAccount = unpackAccount(poolAssociatedTokenAddress, PoolTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedPoolAccount.amount).to.equal(BigInt(70));
        let userTokenAccount = await banksClient.getAccount(userTokenAddress);
        let unpackedAccount = unpackAccount(userTokenAddress, userTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedAccount.amount).to.equal(BigInt((1_000_000 * 10 ** 6)-70));
    });*/

    it("Borrow asset", async () => {
        const [userAddress] = PublicKey.findProgramAddressSync([userOne.publicKey.toBuffer()], puppetProgram.programId);
        const [poolSolConfigAddress] = await PublicKey.findProgramAddressSync([
            SOL.toBuffer(),
        ],puppetProgram.programId);
        
        await puppetProgram.methods.borrowAsset(new anchor.BN(30))
            .accounts(
                {payer: userOne.publicKey, mint: SOL, pool: poolSolConfigAddress, 
                    userAccount: userAddress, 
                    poolTokenAccount: poolSOLAssociatedTokenAddress, 
                    priceUpdate: solUsdPriceFeedAccount, 
                    tokenProgram: TOKEN_PROGRAM_ID })
            .signers([userOne])
            .rpc();
        
    });
});


