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
const getAccounts = ({
    tokenMint,
    stakeTokenMint,
    bidTokenMint,
    roundIndex,
    program,
    sealedBidIndex,
}: any) => {

    const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("authority")],
        program.programId
    )

    // can I do this? or does it have to be a keypair
    const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            programAuthority.toBuffer(),
            Buffer.from("token-mint"),
        ],
        program.programId
    )

    const programTokenAccount = getAssociatedTokenAddressSync(
        programTokenMint,
        programAuthority,
        true
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

    const [marketplaceMatchers] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("marketplace-matchers"),
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

    const [tickBidRound] = tokenMint != undefined && roundIndex != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from(roundIndex.toString()),
            session.toBuffer(),
            Buffer.from("tick-bid-round"),
        ],
        program.programId
    ) : [undefined]

    const [sessionTickBidLeaderBoard] = tokenMint != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            session.toBuffer(),
            Buffer.from("tick-bid-leader-board"),
        ],
        program.programId
    ) : [undefined]

    const [sessionMarketplace] = tokenMint != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            session.toBuffer(),
            Buffer.from("session-marketplace"),
        ],
        program.programId
    ) : [undefined]

    const [vestedConfigBySession] = tokenMint != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            session.toBuffer(),
            Buffer.from("vested-config"),
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


    const [sealedBidAccount] = sealedBidIndex != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from(sealedBidIndex.toString()),
            session.toBuffer(),
            Buffer.from("sealed-bid-by-index"),
        ],
        program.programId
    ) : [undefined]



    return {
        programAuthority,
        programTokenMint,
        programTokenAccount,
        indexerStatus,
        enqueueSessionIndexer,
        activeSessionIndexer,
        session,
        sealedBidRound,
        commitLeaderBoard,
        commitQueue,
        sealedBidTokenStakeAccount,
        commitTokenAccount,
        tickBidRound,
        sessionTickBidLeaderBoard,
        sessionMarketplace,
        marketplaceMatchers,
        vestedConfigBySession,
        sealedBidAccount,
    }
}

// const confirmTransaction = async () => {
//     const latestBlockHash = await connection.getLatestBlockhash()
//     await connection.confirmTransaction({
//         blockhash: latestBlockHash.blockhash,
//         lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
//         signature: tx,
//     });
// }

// part of deployment process
const init = async ({
    connection,
    authority,
    program,
    web3,

}) => {

    const {
        programAuthority,
        indexerStatus,
        enqueueSessionIndexer,
        activeSessionIndexer,
        marketplaceMatchers,

        programTokenMint,
        programTokenAccount,
    } = getAccounts({ tokenMint: undefined, program: program })


    const tx = await program.methods
        .initialize()
        .accounts({
            authority: authority.publicKey,
            newAuthority: programAuthority,
            newIndexerStatus: indexerStatus,
            newActiveSessionIndexer: activeSessionIndexer,
            newEnqueueSessionIndexer: enqueueSessionIndexer,
            newMarketplaceMatchers: marketplaceMatchers,

            newTokenMint: programTokenMint,
            newAuthorityTokenAccount: programTokenAccount,

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

// should come back to this. right now there is only a single top level
// instances of this account being created...
// should there be only one instances or multiple instances at the session level?
// is being created for the bid token... need add validation that this
// can only be created with a valid bid token which is USDC ATM.
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

const createTickBidRound = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,
    bidTokenMint,
    roundIndex,
}) => {

    const {
        session,
        tickBidRound,
    } = getAccounts({
        tokenMint,
        bidTokenMint,
        roundIndex,
        program
    })


    const tx = await program.methods
        .createTickBidRound()
        .accounts({
            authority: authority.publicKey,
            newTickBidRound: tickBidRound,
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

const createSessionTickBidLeaderBoard = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,

}) => {

    const {
        session,
        sessionTickBidLeaderBoard,
    } = getAccounts({
        tokenMint,
        program
    })


    const tx = await program.methods
        .createSessionTickBidLeaderBoard()
        .accounts({
            authority: authority.publicKey,
            newTickBidLeaderBoard: sessionTickBidLeaderBoard,
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

const createSessionMarketplace = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,

}) => {

    const {
        session,
        sessionMarketplace,
    } = getAccounts({
        tokenMint,
        program
    })


    const tx = await program.methods
        .createSessionMarketplace()
        .accounts({
            authority: authority.publicKey,
            newMarketplacePositions: sessionMarketplace,
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

const createVestedConfigBySession = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,

}) => {

    const {
        session,
        vestedConfigBySession,
    } = getAccounts({
        tokenMint,
        program
    })


    const tx = await program.methods
        .createVestedConfigBySession()
        .accounts({
            authority: authority.publicKey,
            newVestedConfig: vestedConfigBySession,
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

const submitSealedBid = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,
    stakeTokenMint,
    input,
}) => {

    const {
        session,
        sealedBidAccount,
        programAuthority,
        sealedBidRound,
        sealedBidTokenStakeAccount
    } = getAccounts({
        tokenMint,
        program,
        stakeTokenMint,
        sealedBidIndex: input.index,
    })

    const tx = await program.methods
        .submitSealedBid(input.commitHash)
        .accounts({
            authority: authority.publicKey,

            newSealedBidByIndex: sealedBidAccount,
            sealedBidRound: sealedBidRound,

            bidderTokenAccount: input.bidderStakeTokenAccount,
            sessionStakeTokenAccount: sealedBidTokenStakeAccount,
            tokenMint: tokenMint.mint.publicKey,

            programAuthority: programAuthority,
            session: session,
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

const submitUnsealedBid = async ({
    connection,
    authority,
    program,
    tokenMint,
    stakeTokenMint,
    input,
}) => {

    const {
        session,
        sealedBidAccount,
        sealedBidRound,
        commitLeaderBoard,
    } = getAccounts({
        tokenMint,
        program,
        stakeTokenMint,
        sealedBidIndex: input.index,
    })

    const data = await program.account.commitLeaderBoard.fetch(commitLeaderBoard)
    // console.log(data.pool.list)

    const list = data.pool.total && new LinkedList(data)
    const index = data.pool.total && list.search(new Node({ position: { amount: input.amount, index: input.index } }))

    // console.log({ index })
    const tx = await program.methods
        .submitUnsealedBid(
            input.amount,
            index,
            input.secret,
        )
        .accounts({
            authority: authority.publicKey,
            sealedBidByIndex: sealedBidAccount,
            sealedBidRound: sealedBidRound,
            commitLeaderBoard: commitLeaderBoard,
            session: session,
        })
        .signers([authority])
        .rpc();

    const latestBlockHash = await connection.getLatestBlockhash()
    await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
    });

    // const d = await program.account.commitLeaderBoard.fetch(commitLeaderBoard)

    // console.log(d)
    // console.log(d)

}

const getCommitLeaderBoard = async ({
    program,
    connection,
    tokenMint,

}) => {

    const {
        commitLeaderBoard,
    } = getAccounts({
        tokenMint,
        program,
    })

    const tx = await program.account.commitLeaderBoard.fetch(commitLeaderBoard)
    const latestBlockHash = await connection.getLatestBlockhash()
    await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
    });

}

class Position {

    bidderIndex: number;
    amount: anchor.BN;

    constructor(position) {
        this.bidderIndex = position.bidderIndex
        this.amount = position.amount
    }
}

class Node {

    index?: number;
    prev?: number;
    next?: number;
    position: Position;

    constructor(node) {
        this.index = node.index
        this.prev = node.prev
        this.next = node.next
        this.position = new Position(node.position)
    }
}

class LinkedList {

    head?: number;
    tail?: number;
    current: number;
    list: Node[];

    constructor({ pool }) {
        // console.log("LIST", pool.list)
        this.head = pool.head
        this.tail = pool.tail
        this.current = this.head
        this.list = pool.list.map(node => {
            return new Node(node)
        })
    }

    next() {
        if (this.list[this.current].next == undefined) {
            this.current = undefined
            return
        }

        this.current = this.list[this.current].next
    }

    prev() {
        if (this.list[this.current].prev == undefined) {
            return
        }

        this.current = this.list[this.current].prev
    }

    get() {
        return this.list[this.current]
    }


    // liner search... can improve, but is good enough for now.
    search(node: Node) {
        this.current = this.head
        while (this.isFound(node)) {
            this.next()
        }

        return this.current !== undefined ? this.current : this.list.length
    }

    isFound(node: Node) {
        // console.log(
        //     this.current,
        //     this.current !== undefined
        //     && node.position.amount.lte(this.get().position.amount),
        //     node.position.amount.toString(),
        //     this.current !== undefined
        //     && this.get().position.amount.toString()
        // )

        return this.current !== undefined
            && node.position.amount.lte(this.get().position.amount)
    }
}


const submitCommitBid = async ({
    connection,
    authority,
    program,
    tokenMint,
    stakeTokenMint,
    bidTokenMint,
    input,
}) => {

    const {
        session,
        sealedBidAccount,
        programAuthority,
        sealedBidRound,
        commitTokenAccount,
        commitLeaderBoard,
        commitQueue,
    } = getAccounts({
        tokenMint,
        program,
        stakeTokenMint,
        bidTokenMint,
        sealedBidIndex: input.index,
    })

    // console.log(commitTokenAccount, bidTokenMint)

    const tx = await program.methods
        .submitCommitBid()
        .accounts({

            authority: authority.publicKey,

            sealedBidByIndex: sealedBidAccount,
            sealedBidRound: sealedBidRound,

            bidderTokenAccount: input.bidTokenAccount,
            sessionCommitTokenAccount: commitTokenAccount,


            commitLeaderBoard: commitLeaderBoard,
            commitQueue: commitQueue,

            tokenMint: tokenMint.mint.publicKey,

            programAuthority: programAuthority,
            session: session,

            tokenProgram: TOKEN_PROGRAM_ID,
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
    createCommitTokenAccount,

    // CREATE SESSION
    createSession,
    // SESSION -> CREATE SEALED-BID SYSTEM

    createSessionSealedBidRound,
    createSessionCommitLeaderBoard,
    createSessionCommitQueue,
    createSealedBidTokenStakeAccount,

    // SESSION -> CREATE TICK-BID SYSTEM
    createTickBidRound,
    createSessionTickBidLeaderBoard,
    // may need leader baord for each round
    createSessionMarketplace,
    createVestedConfigBySession,

    // interact with sealed bid round
    submitSealedBid,
    submitUnsealedBid,
    submitCommitBid,

    // interact with tick bid round
    // register
    // openBid,
    // executeBid,

    getCommitLeaderBoard,

    getAccounts,
}


// TOKEN ACCOUNTS
//  program authority
//  - program token mint
//      - staking account
//          - marketplace matchers
//          - sealed bid commit stake token account
//  session
//  - USDC Token mint / Sol token mint / stable coin token mint
//      - commit bid / commit queue -> one instance or multple instances per session
//      - tick bid / funding for project
//  - launch project token mint
//      - tick bid token allocation
//      - vested token
//  - sealed bid commit stake -> program token mint / stable coin token mint
//      