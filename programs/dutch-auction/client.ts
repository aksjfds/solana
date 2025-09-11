import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { DutchAuction } from "../../target/types/dutch_auction";
import { Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import { createInitializeMint2Instruction, getMinimumBalanceForRentExemptMint, MINT_SIZE, TOKEN_PROGRAM_ID } from "@solana/spl-token";

// 
// 
// 
// 
// 
// 
// ----------

anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.dutch_auction as Program<DutchAuction>;
const connection = program.provider.connection;

// 
// 
// 
// 
// 
describe("entrypoint", () => {
    const payer = program.provider.wallet.payer;
    const mint = Keypair.generate();
    console.log(payer.publicKey.toBase58());
    console.log(mint.publicKey.toBase58());

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
            mint.publicKey, 0, payer.publicKey, null
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

    it("start auction", async () => {

        const info = Keypair.generate();

        const auctionInfo = {
            startPrice: new anchor.BN(LAMPORTS_PER_SOL * 100),
            endPrice: new anchor.BN(LAMPORTS_PER_SOL * 50),
            duration: new anchor.BN(2 * 60 * 60 * 1000),
            dropInterval: new anchor.BN(5 * 60 * 1000),
            dropStep: new anchor.BN(LAMPORTS_PER_SOL * 10)
        };

        const tx = await program.methods.start(
            auctionInfo.startPrice,
            auctionInfo.endPrice,
            auctionInfo.duration,
            auctionInfo.dropInterval,
            auctionInfo.dropStep
        )
            .accounts({ mint: mint.publicKey })
            .rpc();

        // printLogs(tx);
    })

    // 
    // 
    // 
    // 

    const bob = Keypair.generate();
    it("bid", async () => {
        const airdropSig = await connection.requestAirdrop(
            bob.publicKey, LAMPORTS_PER_SOL * 200
        );
        await connection.confirmTransaction({
            signature: airdropSig,
            ...(await connection.getLatestBlockhash())
        });

        const tx = await program.methods
            .bid(new anchor.BN(LAMPORTS_PER_SOL * 120))
            .accounts({
                mint: mint.publicKey,
                seller: payer.publicKey,
                payer: bob.publicKey,
            })
            .signers([bob])
            .rpc();

        // printLogs(tx);
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