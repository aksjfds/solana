import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { NftExchange } from "../../target/types/nft_exchange";
import { Keypair, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import { ACCOUNT_SIZE, createInitializeAccount3Instruction, createInitializeMint2Instruction, createMintToInstruction, getMinimumBalanceForRentExemptAccount, getMinimumBalanceForRentExemptMint, MINT_SIZE, TOKEN_PROGRAM_ID } from "@solana/spl-token";


// 
// 
// 
// 
// 
// 
// ----------

anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.nft_exchange as Program<NftExchange>;
const connection = program.provider.connection;

const payer = program.provider.wallet.payer;
const mint = Keypair.generate();

console.log(payer.publicKey.toBase58());
console.log(mint.publicKey.toBase58());



describe("NFT-Exchange", () => {
  it("create mint account", async () => {
    const createAccIx = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mint.publicKey,
      space: MINT_SIZE,
      lamports: await getMinimumBalanceForRentExemptMint(connection, "processed"),
      programId: TOKEN_PROGRAM_ID
    });

    const initMintIx = createInitializeMint2Instruction(
      mint.publicKey, 0, payer.publicKey, null
    );

    const tx = new Transaction().add(createAccIx, initMintIx);
    const signature = await sendAndConfirmTransaction(
      connection, tx, [payer, mint]
    );

    // printLogs(signature);
  })

  const nft = Keypair.generate();
  console.log(nft.publicKey.toBase58());
  
  it("create nft", async () => {
    const createAccIx = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: nft.publicKey,
      space: ACCOUNT_SIZE,
      lamports: await getMinimumBalanceForRentExemptAccount(connection),
      programId: TOKEN_PROGRAM_ID
    });

    const initTokenAccIx = createInitializeAccount3Instruction(
      nft.publicKey, mint.publicKey, payer.publicKey, TOKEN_PROGRAM_ID
    );

    const mintIx = createMintToInstruction(
      mint.publicKey, nft.publicKey, payer.publicKey, 1, [],
      TOKEN_PROGRAM_ID
    );

    const tx = new Transaction().add(createAccIx, initTokenAccIx, mintIx);
    const signature = await sendAndConfirmTransaction(
      connection, tx, [payer, nft]
    );

  })

  it("sell", async () => {
    const tx = await program.methods
      .sell(new anchor.BN(100))
      .accounts({
        seller: payer.publicKey,
        mint: mint.publicKey,
        nftAccount: nft.publicKey
      })
      .signers([payer])
      .rpc({ commitment: "confirmed" });

    printLogs(tx);
  });
});


// printLogs
const printLogs = (signature: string) => {
  connection.getParsedTransaction(signature, "confirmed").then(res => {
    const logMessages = res.meta.logMessages;
    console.log(logMessages);
  });
}