import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { Faucet } from "../../target/types/faucet";
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
const program = anchor.workspace.Faucet as Program<Faucet>;
const connection = program.provider.connection;

// 
// 
// 
// 
// 
describe("Faucet", () => {
  const payer = program.provider.wallet.payer;
  const mint = Keypair.generate();

  // 
  // 
  // 
  // 

  it("create mint account", async () => {
    const createAccIx = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mint.publicKey,
      space: MINT_SIZE,
      lamports: await getMinimumBalanceForRentExemptMint(connection, "processed"),
      programId: TOKEN_PROGRAM_ID
    });

    const initMintIx = createInitializeMint2Instruction(
      mint.publicKey, 2, payer.publicKey, null
    );

    const tx = new Transaction().add(createAccIx, initMintIx);
    const signature = await sendAndConfirmTransaction(
      connection, tx, [payer, mint]
    );
    // printLogs(signature);
  })

  // 
  // 
  // 
  // 

  // 创建水龙头token account
  it("create faucet", async () => {
    const tx = await program.methods.createFaucet(new anchor.BN(3000))
      .accounts({
        mint: mint.publicKey,
        payer: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const [faucet, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("faucet")], program.programId
    );

    const tokenAccount = await getAccount(
      connection, faucet, "processed", TOKEN_PROGRAM_ID
    );
    // console.log(tokenAccount.amount);
  })

  // 
  // 
  // 
  // 

  it("request_tokens", async () => {
    const bob = Keypair.generate();

    const airdropSig = await connection.requestAirdrop(
      bob.publicKey, LAMPORTS_PER_SOL * 100
    );
    await connection.confirmTransaction({
      signature: airdropSig,
      ...(await connection.getLatestBlockhash())
    });

    const tx = await program.methods.requestTokens()
      .accounts({
        payer: bob.publicKey,
        mint: mint.publicKey,
      }).signers([bob])
      .rpc();

    const [pda, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("pda"), program.programId.toBuffer()], program.programId
    );

    const pdaAccount = await getAccount(
      connection, pda, "processed"
    );
    console.log(pdaAccount);
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