import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { Solana } from "../../target/types/solana";

anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.solana as Program<Solana>;
const connection = program.provider.connection;

describe("entrypoint", () => {
    it("Is initialized!", async () => {
        const amount = new anchor.BN(300);
        console.log(amount);

        const tx = await program.methods.initialize(amount).rpc({ commitment: "processed" });
        // await printLogs(tx);
    });
});


// printLogs
const printLogs = async (signature: string) => {
    const res = await connection.getParsedTransaction(signature, "confirmed");

    const logMessages = res.meta.logMessages
        .filter((message: string) => message.includes("log"));
    console.log(logMessages);
}