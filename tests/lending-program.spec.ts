import { startAnchor} from 'solana-bankrun';
import { BankrunProvider } from 'anchor-bankrun';
import { PublicKey, Keypair } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { LendingProgram } from "../target/types/lending_program";
import { expect } from 'chai';
import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from '@solana/spl-token';

import { createAssociatedTokenAccount, createMint, mintTo } from './utils';
 
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


    before(async () => {
        const programId = PublicKey.unique()
        userOne = Keypair.generate();
        context = await startAnchor("",[{name:"lending_program", programId: programId}],[])
        provider = new BankrunProvider(context);
        puppetProgram = new Program<LendingProgram>(IDL, provider);
        banksClient = context.banksClient;
        mintAuthority = anchor.web3.Keypair.generate();
        payer = provider.wallet.payer;
    });

    it("Initialize user", async () => {
        const transferTransaction = new anchor.web3.Transaction().add(
            anchor.web3.SystemProgram.transfer({
            fromPubkey: puppetProgram.provider.publicKey,
            toPubkey: userOne.publicKey,
            lamports: transferAmount,
            })
        );
        await provider.sendAndConfirm(transferTransaction, [provider.wallet.payer]);
        await puppetProgram.methods.initializeUser()
            .accounts({payer: userOne.publicKey})
            .signers([userOne])
            .rpc();
        
        const [userAddress] = PublicKey.findProgramAddressSync([userOne.publicKey.toBuffer()], puppetProgram.programId);
        const firstUser = await puppetProgram.account.userAccount.fetch(userAddress);
        
    });

    it("Initialize pool and deposit collateral", async () => {
        const USDC = await createMint(
            banksClient,
            payer,
            payer.publicKey,
            payer.publicKey,
            6
          );
          
        let userUsdcAccount = await createAssociatedTokenAccount(
            banksClient,
            userOne,
            USDC,
            userOne.publicKey
        );

        await mintTo(
            banksClient,
            userOne,
            USDC,
            userUsdcAccount,
            payer,
            1_000_000 * 10 ** 6,
        );
          
        await puppetProgram.methods.initializePool(USDC)
            .accounts({payer: puppetProgram.provider.publicKey, mint: USDC})
            .rpc();

        const [userAddress] = PublicKey.findProgramAddressSync([userOne.publicKey.toBuffer()], puppetProgram.programId);
        const [associatedTokenAddress] = await PublicKey.findProgramAddressSync([
            provider.publicKey.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            USDC.toBuffer(),
        ],SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID)
        
        const [poolConfigAddress] = await PublicKey.findProgramAddressSync([
            Buffer.from("pool"),
            USDC.toBuffer(),
        ],puppetProgram.programId)
        
        await puppetProgram.methods.depositCollateral(new anchor.BN(100))
            .accounts({payer: userOne.publicKey, depositMint: USDC, userAccount: userAddress, poolTokenAccount: associatedTokenAddress, poolConfig:poolConfigAddress})
            .signers([userOne])
            .rpc();
    });
});