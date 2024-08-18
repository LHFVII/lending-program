import { startAnchor} from 'solana-bankrun';
import { BankrunProvider } from 'anchor-bankrun';
import { PublicKey, Keypair } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { LendingProgram } from "../target/types/lending_program";
import { expect } from 'chai';
import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, unpackAccount } from '@solana/spl-token';
import { createAssociatedTokenAccount, createMint, mintTo, mockOracleNoProgram } from './utils';
import { BankrunContextWrapper } from './bankrunConnection';
 
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
    let secondMint;
    let userUsdcAccount;
    let userTokenAddress;
    let userSecondMintTokenAddress;

    let poolUsdcAssociatedTokenAddress;
    let poolSecondMintAssociatedTokenAddress;

    before(async () => {
        const programId = new PublicKey('77B3AdNp6RRzsVMQSWAUVv28RXmA8YJVAfWkimRTwXi6')
        userOne = Keypair.generate();
        context = await startAnchor("",[{name:"lending_program", programId: programId},{name:"pyth", programId: new PublicKey('2Fts5wLxxbQB8wSmDwF8wJJ3GgDg7X9P4qMZvEe5gNSH')}],[])
        provider = new BankrunProvider(context);
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
        secondMint = await createMint(
            banksClient,
            payer,
            payer.publicKey,
            payer.publicKey,
            9
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
        

        await puppetProgram.methods.initializePool(new anchor.BN(1))
            .accounts({payer: puppetProgram.provider.publicKey, mint: USDC})
            .rpc();
        
        await puppetProgram.methods.initializePool(new anchor.BN(1))
            .accounts({payer: puppetProgram.provider.publicKey, mint: secondMint})
            .rpc();
        
        userTokenAddress = await getAssociatedTokenAddressSync(USDC, userOne.publicKey);
        userSecondMintTokenAddress = await getAssociatedTokenAddressSync(secondMint, userOne.publicKey);
        [poolUsdcAssociatedTokenAddress] = await PublicKey.findProgramAddressSync([
            provider.publicKey.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            USDC.toBuffer(),
        ],SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID);
        [poolSecondMintAssociatedTokenAddress] = await PublicKey.findProgramAddressSync([
            provider.publicKey.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            USDC.toBuffer(),
        ],SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID);
    });

    it("Initialize pool and deposit collateral", async () => {
        // Check if the user has the correct amount of USDC
        const bankrunContextWrapper = new BankrunContextWrapper(context);
        const priceFeedAddress = await mockOracleNoProgram(bankrunContextWrapper, 1);
    
        /*let userTokenAccount = await banksClient.getAccount(userTokenAddress);
        let unpackedAccount = unpackAccount(userTokenAddress, userTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedAccount.amount).to.equal(BigInt(1_000_000 * 10 ** 6));

        let PoolTokenAccount = await banksClient.getAccount(poolUsdcAssociatedTokenAddress);
        let unpackedPoolAccount = unpackAccount(poolUsdcAssociatedTokenAddress, PoolTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedPoolAccount.amount).to.equal(BigInt(0));*/

        

        const [userAddress] = PublicKey.findProgramAddressSync([userOne.publicKey.toBuffer()], puppetProgram.programId);
        await puppetProgram.methods.depositCollateral(new anchor.BN(100))
            .accounts({payer: userOne.publicKey, depositMint: USDC, userAccount: userAddress, poolTokenAccount: poolUsdcAssociatedTokenAddress, priceFeed: priceFeedAddress})
            .signers([userOne])
            .rpc();
        
        // Check pool and user USDC balance
        /*PoolTokenAccount = await banksClient.getAccount(poolUsdcAssociatedTokenAddress);
        unpackedPoolAccount = unpackAccount(poolUsdcAssociatedTokenAddress, PoolTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedPoolAccount.amount).to.equal(BigInt(100));
        userTokenAccount = await banksClient.getAccount(userTokenAddress);
        unpackedAccount = unpackAccount(userTokenAddress, userTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedAccount.amount).to.equal(BigInt((1_000_000 * 10 ** 6)-100));*/
    });

    /*it("Withdraw", async () => {
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
            .accounts({payer: payer, collateralMint: USDC, userAccount: userAddress, poolTokenAccount: poolAssociatedTokenAddress, userTokenAccount: userTokenAddress})
            .signers([payer])
            .rpc();
        
            PoolTokenAccount = await banksClient.getAccount(poolAssociatedTokenAddress);
        unpackedPoolAccount = unpackAccount(poolAssociatedTokenAddress, PoolTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedPoolAccount.amount).to.equal(BigInt(70));
        let userTokenAccount = await banksClient.getAccount(userTokenAddress);
        let unpackedAccount = unpackAccount(userTokenAddress, userTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedAccount.amount).to.equal(BigInt((1_000_000 * 10 ** 6)-70));
    });*/
});


