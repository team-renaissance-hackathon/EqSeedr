console.log("test, can we stop the IDLERROR?")

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LaunchPad } from "../target/types/launch_pad";

import {
    getAssociatedTokenAddressSync,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    getAccount
} from "@solana/spl-token"
// import { associated } from "@coral-xyz/anchor/dist/cjs/utils/pubkey";



// need keypair from env / config
const getAccounts = ({
    tokenMint,
    stakeTokenMint,
    bidTokenMint,
    roundIndex,
    program,
    sealedBidIndex,
    vestedIndex,
    vestedOwner
}: any) => {

    const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("authority")],
        program.programId
    )

    // can I do this? or does it have to be a keypair
    const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            programAuthority.toBuffer(),
            Buffer.from("eqseedr-token-mint"),
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

    const [vestedAccountByIndex] = vestedIndex != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from(vestedIndex.toString()),
            session.toBuffer(),
            Buffer.from("vested-account-by-index"),
        ],
        program.programId
    ) : [undefined]

    const [vestedAccountByOwner] = vestedOwner != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            vestedOwner.toBuffer(),
            session.toBuffer(),
            Buffer.from("vested-account-by-owner"),
        ],
        program.programId
    ) : [undefined]

    const [tokenStakeVault] = session != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            session.toBuffer(),
            programTokenMint.toBuffer(),
            Buffer.from("token-stake-vault"),
        ],
        program.programId
    ) : [undefined]

    const [commitBidVault] = session != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            session.toBuffer(),
            Buffer.from("commit-bid-vault"),
        ],
        program.programId
    ) : [undefined]

    const [vestedTokenEscrow] = session != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            session.toBuffer(),
            Buffer.from("vested-token-escrow"),
        ],
        program.programId
    ) : [undefined]

    const [ventureTokenEscrow] = session != undefined ? anchor.web3.PublicKey.findProgramAddressSync(
        [
            session.toBuffer(),
            Buffer.from("venture-token-escrow"),
        ],
        program.programId
    ) : [undefined]

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
        tokenStakeVault,
        vestedTokenEscrow,
        ventureTokenEscrow,
        commitBidVault,
        tickBidRound,
        sessionTickBidLeaderBoard,
        sessionMarketplace,
        marketplaceMatchers,
        vestedConfigBySession,
        sealedBidAccount,
        vestedAccountByIndex,
        vestedAccountByOwner,
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
        // marketplaceMatchers,

        programTokenMint,
        programTokenAccount,
    } = getAccounts({ tokenMint: undefined, program: program })

    const [programTokenVault] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            programAuthority.toBuffer(),
            Buffer.from("program-token-vault")
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
            // newMarketplaceMatchers: marketplaceMatchers,

            newTokenMint: programTokenMint,
            // newAuthorityTokenAccount: programTokenAccount,
            newProgramTokenVault: programTokenVault,

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

const mintTokens = async ({
    provider,
    program,
    receipent,
    payer,
    amount
}) => {


    const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("authority")],
        program.programId
    )

    const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            programAuthority.toBuffer(),
            Buffer.from("eqseedr-token-mint"),
        ],
        program.programId
    )

    const [programTokenVault] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            programAuthority.toBuffer(),
            Buffer.from("program-token-vault")
        ],
        program.programId
    )

    const tx = await program.methods
        .mintTokens(amount)
        .accounts({
            // if there is no explicit signer,
            // then .signers([]) is empty
            // but when that happens, who pays the fees?
            signer: payer.publicKey,
            programAuthority: programAuthority,
            // tokenMint: programTokenMint,
            //   receipent: programTokenAccount,
            receipent,


            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([payer])
        .rpc();

    const latestBlockHash = await provider.connection.getLatestBlockhash()
    await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
    });


}

const addBidTokenMint = async ({
    connection,
    program,
    authority,
    bidTokenMint

}) => {

    const {
        programAuthority,
    } = getAccounts({ program })

    const tx = await program.methods
        .addBidTokenMint()
        .accounts({
            authority: authority.publicKey,
            programAuthority: programAuthority,
            tokenMint: bidTokenMint.mint.publicKey,
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
        // .createSession({
        //     tokenName: "",
        //     ...input
        // })
        .createSession(input)
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

const createCommitLeaderBoard = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,
}) => {

    const {
        session,
        commitLeaderBoard,
        sealedBidRound,
    } = getAccounts({ tokenMint, program: program })

    const tx = await program.methods
        .createCommitLeaderBoard()
        .accounts({
            authority: authority.publicKey,
            newCommitLeaderBoard: commitLeaderBoard,
            session: session,
            sealedBidRound: sealedBidRound,
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

const reallocateCommitLeaderBoard = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,
}) => {

    const {
        session,
        commitLeaderBoard,
        sealedBidRound,
    } = getAccounts({ tokenMint, program: program })

    const tx = await program.methods
        .reallocateCommitLeaderBoard()
        .accounts({
            authority: authority.publicKey,
            commitLeaderBoard: commitLeaderBoard,
            sealedBidRound: sealedBidRound,
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

const createTokenStakeVault = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,
    stakeTokenMint,
}) => {

    const {
        session,
        tokenStakeVault,
    } = getAccounts({ tokenMint, stakeTokenMint, program })

    const tx = await program.methods
        .createTokenStakeVault()
        .accounts({
            authority: authority.publicKey,
            newTokenStakeVault: tokenStakeVault,

            session: session,

            ventureTokenMint: tokenMint.mint.publicKey,
            tokenStakeMint: stakeTokenMint.mint,

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

const createCommitBidVault = async ({
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
        commitBidVault,
    } = getAccounts({ tokenMint, bidTokenMint, program })

    const tx = await program.methods
        .createCommitBidVault()
        .accounts({
            authority: authority.publicKey,
            programAuthority: programAuthority,
            newCommitBidVault: commitBidVault,
            session: session,
            bidTokenMint: bidTokenMint.mint.publicKey,
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

const createVestedTokenEscrow = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,

}) => {

    const {
        session,
        vestedTokenEscrow,
    } = getAccounts({
        tokenMint,
        program
    })




    const tx = await program.methods
        .createVestedTokenEscrow()
        .accounts({
            authority: authority.publicKey,

            newVestedTokenEscrow: vestedTokenEscrow,

            session: session,
            tokenMint: tokenMint.mint.publicKey,

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

const createVentureTokenEscrow = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,
    bidTokenMint,

}) => {

    const {
        session,
        ventureTokenEscrow,
    } = getAccounts({
        tokenMint,
        program
    })

    const tx = await program.methods
        .createVentureTokenEscrow()
        .accounts({
            authority: authority.publicKey,

            newVentureTokenEscrow: ventureTokenEscrow,

            session: session,
            bidTokenMint: bidTokenMint.mint.publicKey,

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

const createVestedConfig = async ({
    connection,
    authority,
    program,
    web3,
    tokenMint,

}) => {

    const {
        session,
        vestedConfigBySession,
        vestedTokenEscrow,
    } = getAccounts({
        tokenMint,
        program
    })

    const [escrowAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            tokenMint.mint.publicKey.toBuffer(),
            Buffer.from("escrow"),
        ],
        program.programId
    )




    const tx = await program.methods
        .createVestedConfig()
        .accounts({
            authority: authority.publicKey,
            escrowAuthority: escrowAuthority,
            vestedTokenEscrow: vestedTokenEscrow,
            newVestedConfig: vestedConfigBySession,
            session: session,
            tokenMint: tokenMint.mint.publicKey,

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
        tokenStakeVault,
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

            // bidderTokenStake
            bidderTokenAccount: input.bidderStakeTokenAccount,
            tokenStakeVault: tokenStakeVault,
            tokenStakeMint: stakeTokenMint.mint,

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

    // console.log(sealedBidAccount)

    const list = data.pool.total && new LinkedList(data)
    const index = data.pool.total && list.search(new Node({ position: { amount: input.amount, index: input.index } }))

    console.log({ index })
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

    const d = await program.account.commitLeaderBoard.fetch(commitLeaderBoard)

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
        commitBidVault,
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
            commitBidVault: commitBidVault,


            commitLeaderBoard: commitLeaderBoard,
            commitQueue: commitQueue,

            tokenMint: bidTokenMint.mint.publicKey,

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

const sessionRegistration = async ({
    connection,
    web3,
    authority,
    program,
    tokenMint,
    bidTokenMint,
    input,
}) => {

    const {
        session,
        vestedConfigBySession,
        vestedAccountByIndex,
        vestedAccountByOwner,
    } = getAccounts({
        tokenMint,
        program,
        bidTokenMint,
        vestedIndex: input.vestedIndex,
        vestedOwner: input.vestedOwner,
    })

    console.log("VESTED ACCOUNT BY OWNER::", vestedAccountByOwner)

    const tx = await program.methods
        .sessionRegistration()
        .accounts({
            signer: authority.publicKey,
            authority: authority.publicKey,

            newVestedAccountByIndex: vestedAccountByIndex,
            newVestedAccountByOwner: vestedAccountByOwner,

            vestedConfig: vestedConfigBySession,

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

    // const data = await program.account.vestedAccountByOwner.fetch(vestedAccountByOwner)

    // console.log("VESTED ACCOUNT BY OWNER:: ")
    // console.log("VESTED ACCOUNT BY OWNER:: ", data)
}

const openBid = async ({
    connection,
    web3,
    authority,
    program,
    tokenMint,
    bidTokenMint,
    input,
}) => {


    const {
        session,
    } = getAccounts({
        tokenMint,
        program,
        bidTokenMint,
        vestedIndex: input.vestedIndex,
        vestedOwner: input.vestedOwner,
    })

    const roundIndex = await program.account.session.fetch(session)

    const {

        vestedAccountByIndex,
        vestedAccountByOwner,
        vestedConfigBySession,
        tickBidRound,

        ventureTokenEscrow,
        commitQueue,
        commitBidVault,
    } = getAccounts({
        tokenMint,
        program,
        bidTokenMint,
        vestedIndex: input.vestedIndex,
        vestedOwner: input.vestedOwner,
        roundIndex: roundIndex.currentRound,
    })

    const tx = await program.methods
        .openBid()
        .accounts({
            signer: authority.publicKey,
            authority: authority.publicKey,

            session,
            tickBidRound,
            commitQueue,

            vestedAccountByIndex: vestedAccountByIndex,
            vestedAccountByOwner: vestedAccountByOwner,

            commitBidVault,
            ventureTokenEscrow: ventureTokenEscrow,

            vestedConfig: vestedConfigBySession,

            bidTokenMint: bidTokenMint.mint.publicKey,

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

const executeBid = async ({
    connection,
    web3,
    authority,
    program,
    tokenMint,
    bidTokenMint,
    input,
}) => {

    const {
        session,
    } = getAccounts({
        tokenMint,
        program,
        bidTokenMint,
        vestedIndex: input.vestedIndex,
        vestedOwner: input.vestedOwner,
    })

    const roundIndex = await program.account.session.fetch(session)

    const {

        vestedAccountByIndex,
        vestedAccountByOwner,
        vestedConfigBySession,
        tickBidRound,

        ventureTokenEscrow,
    } = getAccounts({
        tokenMint,
        program,
        bidTokenMint,
        vestedIndex: input.vestedIndex,
        vestedOwner: input.vestedOwner,
        // roundIndex: roundIndex.currentRound,
        roundIndex: 1,

    })

    const before = await program.account.tickBidRound.fetch(tickBidRound)

    const tx = await program.methods
        .executeBid()
        .accounts({
            bidAuthority: authority.publicKey,

            vestedAccountByIndex: vestedAccountByIndex,
            vestedAccountByOwner: vestedAccountByOwner,

            session,
            tickBidRound,
            vestedConfig: vestedConfigBySession,

            bidAta: input.tokenAccount,
            ventureTokenEscrow: ventureTokenEscrow,


            bidTokenMint: bidTokenMint.mint.publicKey,

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

    const after = await program.account.tickBidRound.fetch(tickBidRound)

    console.log(" ")
    console.log("DATA::: ")
    console.log(before.lastMarketBid.toNumber())
    console.log(after.lastMarketBid.toNumber())

}


export const script = {
    init,
    addBidTokenMint,
    createCommitBidVault,

    // CREATE SESSION
    createSession,
    // SESSION -> CREATE SEALED-BID SYSTEM

    createSessionSealedBidRound,

    createCommitLeaderBoard,
    reallocateCommitLeaderBoard,

    createSessionCommitQueue,
    createTokenStakeVault,

    // SESSION -> CREATE TICK-BID SYSTEM
    createTickBidRound,
    createSessionTickBidLeaderBoard,
    // may need leader baord for each round
    createSessionMarketplace,
    createVestedTokenEscrow,
    createVestedConfig,

    // interact with sealed bid round
    submitSealedBid,
    submitUnsealedBid,
    submitCommitBid,

    // interact with tick bid round
    // register
    sessionRegistration,
    openBid,
    executeBid,

    getCommitLeaderBoard,

    getAccounts,

    createVentureTokenEscrow
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