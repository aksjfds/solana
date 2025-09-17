import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { MerkleTree } from "../../target/types/merkle_tree";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import { createInitializeMint2Instruction, getAccount, getMinimumBalanceForRentExemptMint, MINT_SIZE, TOKEN_PROGRAM_ID } from "@solana/spl-token";

// 
// 
// 
// 
// 
// 
// ----------

anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.merkle_tree as Program<MerkleTree>;
const connection = program.provider.connection;

// 
// 
// 
// 
// 
describe("MerkleTree", () => {
  const payer = program.provider.wallet.payer;
  const mint = Keypair.generate();

  // 
  // 
  // 
  // 

  it("init", async () => {
    const tx = await program.methods.initialize().rpc({ commitment: "confirmed" });

    printLogs(tx);
  })


});

// 
// 
// 
// 

const printLogs = (signature: string) => {
  connection.getParsedTransaction(signature, "confirmed").then(res => {
    const logMessages = res.meta.logMessages;
    console.log(logMessages);
  });
}