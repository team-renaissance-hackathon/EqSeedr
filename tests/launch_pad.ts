import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LaunchPad } from "../target/types/launch_pad";

import { createHash } from "crypto";

import {
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  createAssociatedTokenAccountInstruction,
  createInitializeMintInstruction,
  getMinimumBalanceForRentExemptMint,
  createMintToInstruction,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token"

import { script } from "../app/script";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";


const {
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
} = anchor.web3

class Token {
  mint: anchor.web3.Keypair;
  mintAuthority: anchor.web3.Keypair;
  freezeAuthority: anchor.web3.Keypair;
  supply: number;
  decimals: number;
  isInitialized: boolean;


  createMint = async (
    connection: anchor.web3.Connection,
    payer: anchor.web3.Keypair,
    mintAuthority,
    tab,
  ) => {

    this.mint = anchor.web3.Keypair.generate()
    this.mintAuthority = mintAuthority || anchor.web3.Keypair.generate()
    this.freezeAuthority = mintAuthority || anchor.web3.Keypair.generate()
    this.supply = 0
    this.decimals = 9
    this.isInitialized = false

    const lamports = await getMinimumBalanceForRentExemptMint(connection);
    const blockhash = await connection.getLatestBlockhash()

    const transaction = new Transaction({ ...blockhash, feePayer: payer.publicKey }).add(

      SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: this.mint.publicKey,
        space: MINT_SIZE,
        lamports,
        programId: TOKEN_PROGRAM_ID,
      }),

      createInitializeMintInstruction(
        this.mint.publicKey,
        this.decimals,
        this.mintAuthority.publicKey,
        this.freezeAuthority.publicKey
      )
    )


    const tx = await sendAndConfirmTransaction(connection, transaction, [payer, this.mint])

    await connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, "confirmed")

    console.log("\t".repeat(tab), "TOKEN MINT CREATED")

    await this.mintToken({ connection, payer, tab })
  }

  createTokenAccount = async ({ connection, payer, tab }) => {

    // console.log("\t".repeat(tab), "CREATE TOKEN ACCOUNT")

    const tokenAccount = await getAssociatedTokenAddress(
      this.mint.publicKey,
      payer.publicKey,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    )

    const transaction = new Transaction().add(
      createAssociatedTokenAccountInstruction(
        payer.publicKey,
        tokenAccount,
        payer.publicKey,
        this.mint.publicKey,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      ))

    const tx = await sendAndConfirmTransaction(connection, transaction, [payer])

    const blockhash = connection.getLatestBlockhash()
    await connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, "confirmed")

    // console.log("\t".repeat(tab), "TOKEN ACCOUNT CREATED")
  }

  mintToken = async ({
    connection,
    payer,
    tab,
  }) => {

    // console.log("\t".repeat(tab), "START MINT")

    await this.createTokenAccount({ connection, payer, tab })

    const tokenAccount = await getAssociatedTokenAddress(
      this.mint.publicKey,
      payer.publicKey,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    )

    const ix = createMintToInstruction(
      this.mint.publicKey,
      tokenAccount,
      this.mintAuthority.publicKey,
      10000 * LAMPORTS_PER_SOL,
    )

    const blockhash = await connection.getLatestBlockhash()
    const transaction = new Transaction(blockhash)
    transaction.add(ix).sign(payer)

    const tx = await sendAndConfirmTransaction(connection, transaction, [payer, this.mintAuthority])

    await connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, "confirmed")

    // console.log("\t".repeat(tab), "TOKEN MINTED")

  }
}


describe("launch_pad", () => {

  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.LaunchPad as Program<LaunchPad>;
  const keypair = anchor.web3.Keypair.generate()
  const bidTokenMint = new Token() // USDC / SOL Token / STABLE token
  const tokenMint = new Token() // session


  before(async () => {
    {
      const tx = await provider.connection.requestAirdrop(
        keypair.publicKey,
        10000 * anchor.web3.LAMPORTS_PER_SOL
      )

      const latestBlockHash = await provider.connection.getLatestBlockhash()
      await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
      });
    }

    {

      const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("authority")],
        program.programId
      )

      await tokenMint.createMint(
        provider.connection,
        keypair,
        undefined,
        // { publicKey: programAuthority },
        1
      )
      const tx = await provider.connection.requestAirdrop(
        tokenMint.mintAuthority.publicKey,
        10000 * anchor.web3.LAMPORTS_PER_SOL
      )

      const latestBlockHash = await provider.connection.getLatestBlockhash()
      await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
      });
    }

    {

      await bidTokenMint.createMint(
        provider.connection,
        keypair, undefined, 1
      )
      const tx = await provider.connection.requestAirdrop(
        bidTokenMint.mintAuthority.publicKey,
        10000 * anchor.web3.LAMPORTS_PER_SOL
      )

      const latestBlockHash = await provider.connection.getLatestBlockhash()
      await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
      });
    }

  })

  it("Is initialized!", async () => {

    await script.init({
      connection: provider.connection,
      authority: keypair,
      program,
      web3: anchor.web3,
    })

  });

  // this should be moved into session for sealed bid?
  // right now only one instance of this is being created
  // multiple instances should  be created for each session.
  // program authority is the authority of account but should that be true?
  // now that I think about it. this is for the commit queue.
  // so then maybe only one instances is okay to exist?
  // and is using a valid bid token mint -> USDC / SOL / Stable coin
  it("Create Commit Token Account", async () => {

    await script.createCommitTokenAccount({
      connection: provider.connection,
      // is just payer and this is the session token mint
      // is that accurate? or should be the bidTokenMint?
      authority: tokenMint.mintAuthority,
      program,
      web3: anchor.web3,
      tokenMint,
      bidTokenMint,
    })

  })

  describe("Session", () => {

    const stakeTokenMint = new Token()
    before(async () => {
      // sealed bid commit stake account
      //  - program token mint
      //  - valid stable coin mint
      //  - sol token mint != native sol
      {

        // using a valid stable coin token mint in this test case
        await stakeTokenMint.createMint(
          provider.connection,
          keypair, undefined, 2
        )
        const tx = await provider.connection.requestAirdrop(
          stakeTokenMint.mintAuthority.publicKey,
          1000 * anchor.web3.LAMPORTS_PER_SOL
        )

        const latestBlockHash = await provider.connection.getLatestBlockhash()
        await provider.connection.confirmTransaction({
          blockhash: latestBlockHash.blockhash,
          lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
          signature: tx,
        });

      }

    })

    describe("Initialize Session State Contracts", () => {

      it("Create New Session", async () => {

        await script.createSession({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          input: {
            tokenName: "EqSeedr",
            launchDate: new anchor.BN(1713613050),
            tokenAllocation: new anchor.BN(1_000_000_000 * 1_000_000),
          }
        })

      })

      it("Create Session Sealed Bid round", async () => {

        await script.createSessionSealedBidRound({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
        })

      })

      it("Create Session Commit Leader Board", async () => {

        await script.createSessionCommitLeaderBoard({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
        })

      })

      it("Create Session Commit Queue", async () => {

        await script.createSessionCommitQueue({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
        })

      })

      it("Create Sealed Bid Token Stake Account", async () => {

        await script.createSealedBidTokenStakeAccount({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          // a valid token mint -> program token -> or USDC / stable token mint
          stakeTokenMint,
        })

      })

      it("Create Tick Bid Rounds", async () => {


        await script.createTickBidRound({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          bidTokenMint,
          roundIndex: 1,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          bidTokenMint,
          roundIndex: 2,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          bidTokenMint,
          roundIndex: 3,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          bidTokenMint,
          roundIndex: 4,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          bidTokenMint,
          roundIndex: 5,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          bidTokenMint,
          roundIndex: 6,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          bidTokenMint,
          roundIndex: 7,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          bidTokenMint,
          roundIndex: 8,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          bidTokenMint,
          roundIndex: 9,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          bidTokenMint,
          roundIndex: 10,
        })

      })

      it("Create Session Tick Bid Leader Board", async () => {

        await script.createSessionTickBidLeaderBoard({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
        })
      })

      it("Create Tick Bid Marketplace", async () => {

        await script.createSessionMarketplace({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
        })
      })

      it("Create Vested Config By Session", async () => {

        await script.createVestedConfigBySession({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
        })
      })

    })

    describe("Interact with Sealed Bid System", () => {

      const users = []

      before(async () => {
        const list = []

        for (let i = 0; i < 1; i++) {

          const keypair = anchor.web3.Keypair.generate()

          const tx = await provider.connection.requestAirdrop(
            keypair.publicKey,
            10000 * anchor.web3.LAMPORTS_PER_SOL
          ).then(tx => {

            return provider.connection.getLatestBlockhash()
              .then(latestBlockHash => {

                return provider.connection.confirmTransaction({
                  blockhash: latestBlockHash.blockhash,
                  lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
                  signature: tx,
                })
              })

          }).then(() => {

            return bidTokenMint
              .mintToken({ connection: provider.connection, payer: keypair, tab: 3 })
          }).then(() => {

            return stakeTokenMint
              .mintToken({ connection: provider.connection, payer: keypair, tab: 3 })
          })

          list.push(tx)


          const passPhrase = "secret-" + i.toString()
          const secret = createHash('sha256')
            // will use session + passPhrase
            .update(keypair.publicKey.toBuffer())
            .update(passPhrase)
            .digest()

          const amount = 100 + (i * 100)
          const commitHash = createHash('sha256')
            .update(amount.toString())
            .update(keypair.publicKey.toBuffer())
            .update(secret)
            .digest()

          users.push({
            keypair,
            amount,
            secret,
            commitHash: new anchor.web3.PublicKey(commitHash),
            index: i + 1,
            bidTokenAccount: getAssociatedTokenAddress(
              bidTokenMint.mint.publicKey,
              keypair.publicKey,
              true,
              TOKEN_PROGRAM_ID,
              ASSOCIATED_TOKEN_PROGRAM_ID
            ),
            bidderStakeTokenAccount: getAssociatedTokenAddress(
              stakeTokenMint.mint.publicKey,
              keypair.publicKey,
              true,
              TOKEN_PROGRAM_ID,
              ASSOCIATED_TOKEN_PROGRAM_ID
            )
          })
        }

        await Promise.all(list)
      })

      it("Submit Sealed Bid", async () => {

        const fn = async (index) => {

          if (index == users.length) {
            return
          }

          await script.submitSealedBid({
            connection: provider.connection,
            authority: users[index].keypair,
            program: program,
            web3: anchor.web3,
            tokenMint,
            stakeTokenMint,
            input: users[index],
          })

          index++
          await fn(index)
        }

        await fn(0)
      })
      // submit unsealed bid
      // submit commit
    })

    describe("Interact with Tick Bid System", () => {

      // before
      // create 15 more users to test the tick bid system

      // registerVestedAccount
      // process commit for round, opens the round
      // execute bid
      // delayed execute bid
      // need to excute many bids quickly to close the tick bid round and enter another tick bid round
      // need to execute all the rounds fast to close the session
    })

    describe("Process Errors", () => { })
  })





});
