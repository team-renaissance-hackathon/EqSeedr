console.log("test, can we stop the IDLERROR?")

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LaunchPad } from "../target/types/launch_pad";



// need keypair from env / config


// part of deployment process
const init = async ({
    connection,
    authority,
    program,
    web3
}) => {

    const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("authority")],
        program.programId
    )

    const [indexerStatus] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("indexer-status"),
            programAuthority.toBuffer(),
        ],
        program.programId
    )

    const [enqueueSessionIndexer] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("enqueue-session-indexer"),
            programAuthority.toBuffer(),
        ],
        program.programId
    )

    const [activeSessionIndexer] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("active-session-indexer"),
            programAuthority.toBuffer(),
        ],
        program.programId
    )

    const tx = await program.methods
        .initialize()
        .accounts({
            authority: authority.publicKey,
            newAuthority: programAuthority,
            newIndexerStatus: indexerStatus,
            newActiveSessionIndexer: activeSessionIndexer,
            newEnqueueSessionIndexer: enqueueSessionIndexer,
            systemProgram: web3.SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

    const latestBlockHash = await connection.getLatestBlockhash()
    await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
    });
}

const createSession = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,
    input
}) => {
    const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("authority")],
        program.programId
    )

    const [indexerStatus] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("indexer-status"),
            programAuthority.toBuffer(),
        ],
        program.programId
    )

    const [session] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            tokenMint.mint.publicKey.toBuffer(),
            Buffer.from("session"),
        ],
        program.programId
    )

    const tx = await program.methods
        .createSession({
            tokenName: "",
            ...input
        })
        .accounts({
            authority: authority.publicKey,
            indexer: indexerStatus,
            newSession: session,
            tokenMint: tokenMint.mint.publicKey,
            systemProgram: web3.SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

    const latestBlockHash = await connection.getLatestBlockhash()
    await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
    });

    // return the session?
}


const createSessionSealedBidRound = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,
}) => {

    const [session] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            tokenMint.mint.publicKey.toBuffer(),
            Buffer.from("session"),
        ],
        program.programId
    )

    const [sealedBidRound] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            session.toBuffer(),
            Buffer.from("sealed-bid-round"),
        ],
        program.programId
    )

    const tx = await program.methods
        .createSessionSealedBidRound()
        .accounts({
            authority: authority.publicKey,
            newSealedBidRound: sealedBidRound,
            session: session,
            tokenMint: tokenMint.mint.publicKey,
            systemProgram: web3.SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

    const latestBlockHash = await connection.getLatestBlockhash()
    await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
    });

}

export const script = {
    init,
    createSession,
    createSessionSealedBidRound,
}