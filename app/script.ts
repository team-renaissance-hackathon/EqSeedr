console.log("test, can we stop the IDLERROR?")

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LaunchPad } from "../target/types/launch_pad";

import {
    getAssociatedTokenAddressSync,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
} from "@solana/spl-token"



// need keypair from env / config
const getAccounts = ({ tokenMint, stakeTokenMint, bidTokenMint, program }: any) => {

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

    const [session] = tokenMint != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            tokenMint.mint.publicKey.toBuffer(),
            Buffer.from("session"),
        ],
        program.programId
    ) : [undefined]

    const [sealedBidRound] = tokenMint != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            session.toBuffer(),
            Buffer.from("sealed-bid-round"),
        ],
        program.programId
    ) : [undefined]

    const [commitLeaderBoard] = tokenMint != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            session.toBuffer(),
            Buffer.from("commit-leader-board"),
        ],
        program.programId
    ) : [undefined]

    const [commitQueue] = tokenMint != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            session.toBuffer(),
            Buffer.from("commit-queue"),
        ],
        program.programId
    ) : [undefined]


    const sealedBidTokenStakeAccount = stakeTokenMint != undefined ? getAssociatedTokenAddressSync(
        stakeTokenMint.mint.publicKey,
        session,
        true
    ) : undefined

    const commitTokenAccount = bidTokenMint != undefined ? getAssociatedTokenAddressSync(
        bidTokenMint.mint.publicKey,
        programAuthority,
        true
    ) : undefined


    return {
        programAuthority,
        indexerStatus,
        enqueueSessionIndexer,
        activeSessionIndexer,
        session,
        sealedBidRound,
        commitLeaderBoard,
        commitQueue,
        sealedBidTokenStakeAccount,
        commitTokenAccount,
    }
}

// part of deployment process
const init = async ({
    connection,
    authority,
    program,
    web3
}) => {

    const {
        programAuthority,
        indexerStatus,
        enqueueSessionIndexer,
        activeSessionIndexer,
    } = getAccounts({ tokenMint: undefined, program: program })

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

    const {
        indexerStatus,
        session,
    } = getAccounts({ tokenMint, program: program })

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

    const {
        session,
        sealedBidRound,
    } = getAccounts({ tokenMint, program: program })

    const tx = await program.methods
        .createSessionSealedBidRound()
        .accounts({
            authority: authority.publicKey,
            newSealedBidRound: sealedBidRound,
            session: session,
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

const createSessionCommitLeaderBoard = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,
}) => {

    const {
        session,
        commitLeaderBoard,
    } = getAccounts({ tokenMint, program: program })

    const tx = await program.methods
        .createSessionCommitLeaderBoard()
        .accounts({
            authority: authority.publicKey,
            newCommitLeaderBoard: commitLeaderBoard,
            session: session,
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

const createSessionCommitQueue = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,
}) => {

    const {
        session,
        commitQueue,
    } = getAccounts({ tokenMint, program: program })

    const tx = await program.methods
        .createSessionCommitQueue()
        .accounts({
            authority: authority.publicKey,
            newCommitQueue: commitQueue,
            session: session,
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

const createSealedBidTokenStakeAccount = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,
    stakeTokenMint,
}) => {

    const {
        session,
        programAuthority,
        sealedBidTokenStakeAccount,
    } = getAccounts({ tokenMint, stakeTokenMint, program })


    const tx = await program.methods
        .createSealedBidTokenStakeAccount()
        .accounts({
            authority: authority.publicKey,
            newSealedBidTokenStakeAccount: sealedBidTokenStakeAccount,
            session: session,
            programAuthority: programAuthority,
            sessionTokenMint: tokenMint.mint.publicKey,
            stakeTokenMint: stakeTokenMint.mint.publicKey,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID,
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

const createCommitTokenAccount = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,
    bidTokenMint,
}) => {

    const {
        session,
        programAuthority,
        commitTokenAccount,
    } = getAccounts({ tokenMint, bidTokenMint, program })

    const tx = await program.methods
        .createCommitTokenAccount()
        .accounts({
            authority: authority.publicKey,
            newCommitTokenAccount: commitTokenAccount,
            session: session,
            programAuthority: programAuthority,
            bidTokenMint: bidTokenMint.mint.publicKey,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID,
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
    createSessionCommitLeaderBoard,
    createSessionCommitQueue,
    createSealedBidTokenStakeAccount,
    createCommitTokenAccount,
}