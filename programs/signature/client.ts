import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { Signature } from "../../target/types/signature";
import { Ed25519Program, Keypair } from "@solana/web3.js";
import { createKeyPairFromBytes, getUtf8Encoder, SignatureBytes, signBytes } from "@solana/kit";

// 
// 
// 
// 
// 
// 
// ----------

anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.signature as Program<Signature>;
const connection = program.provider.connection;

// 
// 
// 
// 
// 
describe("Signature", () => {
  const payer = program.provider.wallet.payer;
  const bob = Keypair.generate();


  // 
  // 
  // 
  // 
  let message: Uint8Array;
  let signature: SignatureBytes;

  it("init", async () => {
    const payersKeyPair = await createKeyPairFromBytes(
      payer.secretKey
    );
    message = new Uint8Array(bob.publicKey.toBuffer());
    signature = await signBytes(payersKeyPair.privateKey, message);
  })

  it("verify", async () => {
    const ed25519Ix = Ed25519Program.createInstructionWithPublicKey({
      publicKey: payer.publicKey.toBytes(), message: message, signature: signature
    });

    const tx = await program.methods
      .verify()
      .accounts({ payer: bob.publicKey })
      .signers([bob])
      .preInstructions([ed25519Ix])
      .rpc({ commitment: "confirmed" });

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