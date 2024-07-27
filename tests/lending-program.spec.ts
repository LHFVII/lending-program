import { startAnchor} from 'solana-bankrun';
import { BankrunProvider } from 'anchor-bankrun';
import { PublicKey, Keypair } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { LendingProgram } from "../target/types/lending_program";
import { expect } from 'chai';
import * as anchor from "@coral-xyz/anchor";
import { createMint } from './utils';
 
const IDL = require("../target/idl/lending_program.json");

const transferAmount = 1_000_000_000;


describe("Create a system account", async () => {
    let userOne;
    let puppetProgram;
    let provider;
    let banksClient;
    let mintAuthority;


    before(async () => {
        const programId = PublicKey.unique()
        userOne = Keypair.generate();
        const context = await startAnchor("",[{name:"lending_program", programId: programId}],[])
        provider = new BankrunProvider(context);
        puppetProgram = new Program<LendingProgram>(IDL, provider);
        banksClient = context.banksClient;
        mintAuthority = anchor.web3.Keypair.generate();
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

    it("Initialize pool", async () => {
        const mint = await createMint(banksClient, provider.wallet.payer, mintAuthority.publicKey, mintAuthority.publicKey, 9);
        await puppetProgram.methods.initializePool()
            .accounts({payer: puppetProgram.provider.publicKey, mint: mint})
            .rpc();
    });

    it("Deposit collateral", async () => {
        const mint = await createMint(banksClient, provider.wallet.payer, mintAuthority.publicKey, mintAuthority.publicKey, 9);
        
    });
});