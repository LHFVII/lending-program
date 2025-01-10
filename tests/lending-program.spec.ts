import { startAnchor } from 'solana-bankrun';
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
import { Connection } from '@solana/web3.js';

const IDL = require("../target/idl/lending_program.json");

const transferAmount = 1_000_000_000;

describe("Create a system account", () => {
    let context;
    let userOne;
    let puppetProgram;
    let provider;
    let banksClient;
    let payer;
    let USDC: PublicKey;
    let SOL: PublicKey;
    let userUsdcAccount;
    let userTokenAddress;

    let poolUsdcAssociatedTokenAddress;
    let bankrunContextWrapper: BankrunContextWrapper;
    let connection

    before(async () => {
        const pyth = new PublicKey('7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE');

        const devnetConnection = new Connection('https://api.devnet.solana.com');
        const accountInfo = await devnetConnection.getAccountInfo(pyth);


        context = await startAnchor('', [{ name: "lending_program", programId: new PublicKey(IDL.address) }], [{
            address: pyth,
            info: accountInfo,
        }]);

        userOne = Keypair.generate();
        provider = new BankrunProvider(context);

        const SOL_PRICE_FEED_ID = '0xeaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a'
        const pythSolanaReceiver = new PythSolanaReceiver({ connection: devnetConnection, wallet: provider.wallet })
        const SOL_USD_PRICE_ACCOUNT = pythSolanaReceiver.getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID)

        bankrunContextWrapper = new BankrunContextWrapper(context);
        connection = bankrunContextWrapper.connection.toConnection();

        puppetProgram = new Program<LendingProgram>(IDL, provider);

        banksClient = context.banksClient;

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
            .accounts({ payer: userOne.publicKey })
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

        await puppetProgram.methods.initializePool(new anchor.BN(10), new anchor.BN(100000))
            .accounts({ payer: puppetProgram.provider.publicKey, mint: USDC, tokenProgram: TOKEN_PROGRAM_ID })
            .rpc();

        await puppetProgram.methods.initializePool(new anchor.BN(10), new anchor.BN(100000))
            .accounts({ payer: puppetProgram.provider.publicKey, mint: SOL, tokenProgram: TOKEN_PROGRAM_ID })
            .rpc();

        userTokenAddress = await getAssociatedTokenAddressSync(USDC, userOne.publicKey);

        [poolUsdcAssociatedTokenAddress] = await PublicKey.findProgramAddressSync([
            Buffer.from('treasury'),
            USDC.toBuffer(),
        ], puppetProgram.programId);
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
            .accounts({ payer: userOne.publicKey, mint: USDC, userAccount: userAddress, userTokenAccount: userTokenAddress, poolTokenAccount: poolUsdcAssociatedTokenAddress, tokenProgram: TOKEN_PROGRAM_ID })
            .signers([userOne])
            .rpc();

        PoolTokenAccount = await banksClient.getAccount(poolUsdcAssociatedTokenAddress);
        unpackedPoolAccount = unpackAccount(poolUsdcAssociatedTokenAddress, PoolTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedPoolAccount.amount).to.equal(BigInt(100));
        userTokenAccount = await banksClient.getAccount(userTokenAddress);
        unpackedAccount = unpackAccount(userTokenAddress, userTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedAccount.amount).to.equal(BigInt((1_000_000 * 10 ** 6) - 100));
    });

    it("Withdraw collateral", async () => {
        let userTokenAccount = await banksClient.getAccount(userTokenAddress);
        let unpackedAccount = unpackAccount(userTokenAddress, userTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedAccount.amount).to.equal(BigInt((1_000_000 * 10 ** 6) - 100));

        let PoolTokenAccount = await banksClient.getAccount(poolUsdcAssociatedTokenAddress);
        let unpackedPoolAccount = unpackAccount(poolUsdcAssociatedTokenAddress, PoolTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedPoolAccount.amount).to.equal(BigInt(100));

        const [userAddress] = PublicKey.findProgramAddressSync([userOne.publicKey.toBuffer()], puppetProgram.programId);

        await puppetProgram.methods.withdrawCollateral(new anchor.BN(100))
            .accounts({ payer: userOne.publicKey, mint: USDC, userAccount: userAddress, userTokenAccount: userTokenAddress, poolTokenAccount: poolUsdcAssociatedTokenAddress, tokenProgram: TOKEN_PROGRAM_ID })
            .signers([userOne])
            .rpc();

        PoolTokenAccount = await banksClient.getAccount(poolUsdcAssociatedTokenAddress);
        unpackedPoolAccount = unpackAccount(poolUsdcAssociatedTokenAddress, PoolTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedPoolAccount.amount).to.equal(BigInt(100));
        userTokenAccount = await banksClient.getAccount(userTokenAddress);
        unpackedAccount = unpackAccount(userTokenAddress, userTokenAccount, TOKEN_PROGRAM_ID);
        expect(unpackedAccount.amount).to.equal(BigInt((1_000_000 * 10 ** 6) - 100));
    });

    it("Absorb loan", async () => {
        let userTokenAccount = await banksClient.getAccount(userTokenAddress);
        let unpackedAccount = unpackAccount(userTokenAddress, userTokenAccount, TOKEN_PROGRAM_ID);

    });
});


