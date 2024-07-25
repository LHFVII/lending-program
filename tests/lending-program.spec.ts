import { startAnchor} from 'solana-bankrun';
import { BankrunProvider } from 'anchor-bankrun';
import { PublicKey, Keypair } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { LendingProgram } from "../target/types/lending_program";
import { expect } from 'chai';
import * as anchor from "@coral-xyz/anchor";
 
const IDL = require("../target/idl/lending_program.json");

const LAMPORTS_PER_SOL = 1000000000;            


describe("Create a system account", async () => {
    it("Bankrun should be able to deploy the program", async () => {
        const programId = PublicKey.unique()
        const userOne = Keypair.generate();
        
        const context = await startAnchor("",[{name:"lending_program", programId: programId}],[])
        const provider = new BankrunProvider(context);
        const transferAmount = 1_000_000_000; // Example amount, adjust as needed
        
      
        const puppetProgram = new Program<LendingProgram>(IDL, provider);

        const transferTransaction = new anchor.web3.Transaction().add(
            anchor.web3.SystemProgram.transfer({
            fromPubkey: puppetProgram.provider.publicKey,
            toPubkey: userOne.publicKey,
            lamports: transferAmount,
            })
        );

        // Send and confirm the transfer transaction
      await provider.sendAndConfirm(transferTransaction, [provider.wallet.payer]);
        
        await puppetProgram.methods.initializeUser()
            .accounts({payer: userOne.publicKey})
            .signers([userOne])
            .rpc();
        
    });
});