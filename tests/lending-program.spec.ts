import { startAnchor} from 'solana-bankrun';
import { BankrunProvider } from 'anchor-bankrun';
import { PublicKey } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { LendingProgram } from "../target/types/lending_program";
import { expect } from 'chai';
import * as anchor from "@coral-xyz/anchor";
 
const IDL = require("../target/idl/lending_program.json");

describe("Create a system account", async () => {
    it("Bankrun should be able to deploy the program", async () => {
        const programId = PublicKey.unique()
        const context = await startAnchor("",[{name:"lending_program", programId: programId}],[])
        const provider = new BankrunProvider(context);
        const puppetProgram = new Program<LendingProgram>(IDL, provider);
        
    });
});