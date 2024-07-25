import { BanksClient } from 'solana-bankrun';
import { SystemProgram, PublicKey, Keypair, Transaction } from "@solana/web3.js";
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