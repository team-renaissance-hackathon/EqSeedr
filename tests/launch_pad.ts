import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LaunchPad } from "../target/types/launch_pad";

import { createHash } from "crypto";

import {
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
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
import { token } from "@coral-xyz/anchor/dist/cjs/utils";


const {
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
  PublicKey,
  Keypair,
} = anchor.web3

const shuffle = (array: string[]) => {
  for (let i = array.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]];
  }
  return array;
};

class Mint {
  keypair?: anchor.web3.Keypair;
  pubkey?: anchor.web3.PublicKey;
}


class Token {
  // mint: anchor.web3.Keypair | anchor.web3.PublicKey;
  // mint: anchor.web3.Keypair;
  mint: Mint;
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
    mint?: anchor.web3.PublicKey,
  ) => {

    const data = new Mint()

    if (mint !== undefined) {
      data.pubkey = mint
    } else {
      data.keypair = anchor.web3.Keypair.generate()
    }

    this.mint = data
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
        newAccountPubkey: this.mint.keypair.publicKey,
        space: MINT_SIZE,
        lamports,
        programId: TOKEN_PROGRAM_ID,
      }),

      createInitializeMintInstruction(
        this.mint.keypair.publicKey,
        this.decimals,
        this.mintAuthority.publicKey,
        this.freezeAuthority.publicKey
      )
    )


    const tx = await sendAndConfirmTransaction(connection, transaction, [payer, this.mint.keypair])

    await connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, "confirmed")

    console.log("\t".repeat(tab), "TOKEN MINT CREATED")

    await this.mintToken({ connection, payer, tab })
  }

  createTokenAccount = async ({ connection, payer, tab }) => {

    console.log("\t".repeat(tab), "CREATE TOKEN ACCOUNT")

    const tokenAccount = await getAssociatedTokenAddress(
      this.mint.pubkey || this.mint.keypair.publicKey,
      // this.mint.keypair.publicKey,

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
        this.mint.pubkey || this.mint.keypair.publicKey,
        // this.mint.keypair.publicKey,

        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      ))

    const tx = await sendAndConfirmTransaction(connection, transaction, [payer])

    const blockhash = connection.getLatestBlockhash()
    await connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, "confirmed")

    console.log("\t".repeat(tab), "TOKEN ACCOUNT CREATED")
  }

  mintToken = async ({
    connection,
    payer,
    tab,
  }) => {

    // console.log("\t".repeat(tab), "START MINT")

    await this.createTokenAccount({ connection, payer, tab })

    const tokenAccount = await getAssociatedTokenAddress(
      this.mint.keypair.publicKey,
      payer.publicKey,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    )

    const ix = createMintToInstruction(
      this.mint.keypair.publicKey,
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
  const tokens = {

    // accepted tokens to bid in auction / stable coins
    bidTokenMint: [{ name: "USDC", token: new Token() }],

    // launchProjectTokenMint
    ventureTokenMint: [{ creator: anchor.web3.Keypair.generate(), token: new Token() }],

    // stakeTokenMint
    programTokenMint: new Token(),
  }
  // const bidTokenMint = new Token() // USDC / SOL Token / STABLE token
  // const tokenMint = new Token() // eqseedr token mint


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

      const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          programAuthority.toBuffer(),
          Buffer.from("eqseedr-token-mint")
        ],
        program.programId
      )

      const mint = new Mint()
      mint.pubkey = programTokenMint
      tokens.programTokenMint.mint = mint

      // await tokenMint.createMint(
      //   provider.connection,
      //   keypair,
      //   undefined,
      //   // { publicKey: programAuthority },
      //   1
      // )

      const tx = await provider.connection.requestAirdrop(
        // tokenMint.mintAuthority.publicKey,
        programTokenMint,
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

      const bidTokenMint = tokens.bidTokenMint.find(token => token.name === "USDC").token

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

  describe("initialize priority accounts", () => {

    it("Is initialized!", async () => {
      await script.init({
        connection: provider.connection,
        authority: keypair,
        program,
        web3: anchor.web3,
      })
    });

    it("Mint Tokens", async () => {

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
        .mintTokens(new anchor.BN(1000 * LAMPORTS_PER_SOL))
        .accounts({
          // if there is no explicit signer,
          // then .signers([]) is empty
          // but when that happens, who pays the fees?
          signer: keypair.publicKey,
          programAuthority: programAuthority,
          // tokenMint: programTokenMint,
          receipent: programTokenVault,

          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([keypair])
        .rpc();

      const latestBlockHash = await provider.connection.getLatestBlockhash()
      await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
      });

    })

    it("Transfer Tokens From Program Token Vault", async () => {
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

      // const receipentATA = getAssociatedTokenAddressSync(
      //   programTokenMint,
      //   keypair.publicKey,
      //   true
      // )

      const receipentATA = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        keypair,
        programTokenMint,
        keypair.publicKey
      )

      const tx = await program.methods
        .transferTokens(new anchor.BN(1 * LAMPORTS_PER_SOL))
        .accounts({
          signer: keypair.publicKey,
          programAuthority: programAuthority,
          // tokenMint: programTokenMint,
          programTokenVault: programTokenVault,
          receipent: receipentATA.address,

          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([keypair])
        .rpc();

      const latestBlockHash = await provider.connection.getLatestBlockhash()
      await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
      });

    })

    it("Add Bid Token Mint", async () => {
      await script.addBidTokenMint({
        connection: provider.connection,
        authority: keypair,
        program,
        bidTokenMint: tokens.bidTokenMint.find(token => token.name === "USDC").token,
      })
    });

  })

  describe("Session", () => {

    // don't need this before
    // const stakeTokenMint = new Token()
    const stakeTokenMint = tokens.programTokenMint
    const bidTokenMint = tokens.bidTokenMint.find(token => token.name === "USDC").token

    before(async () => {
      // sealed bid commit stake account
      //  - program token mint
      //  - valid stable coin mint
      //  - sol token mint != native sol
      {

        // using a valid stable coin token mint in this test case
        // await stakeTokenMint.createMint(
        //   provider.connection,
        //   keypair, undefined, 2
        // )

        const tx = await provider.connection.requestAirdrop(
          // stakeTokenMint.mintAuthority.publicKey,
          stakeTokenMint.mint.pubkey,
          1000 * anchor.web3.LAMPORTS_PER_SOL
        )

        const latestBlockHash = await provider.connection.getLatestBlockhash()
        await provider.connection.confirmTransaction({
          blockhash: latestBlockHash.blockhash,
          lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
          signature: tx,
        });

      }

      console.log(stakeTokenMint.mint.pubkey)


    })

    const target = 15
    const users = []

    before(async () => {

      const list = []

      function random(min, max) { // min and max included 
        return Math.floor(Math.random() * (max - min + 1) + min)
      }

      for (let i = 0; i < target; i++) {

        // investor
        const keypair = anchor.web3.Keypair.generate()

        // investor: AIRDROP TOKENS / SOL 
        const tx = await provider.connection.requestAirdrop(
          // investor requesting SOL
          keypair.publicKey,
          10000 * anchor.web3.LAMPORTS_PER_SOL
        ).then(tx => {

          // WAIT FOR AIRDROP CONFIRMATION
          return provider.connection.getLatestBlockhash()
            .then(latestBlockHash => {

              return provider.connection.confirmTransaction({
                blockhash: latestBlockHash.blockhash,
                lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
                signature: tx,
              })
            })

        }).then(() => {

          // investor: AIRDROP USDC -> BIDTOKENMINT
          return bidTokenMint
            .mintToken({ connection: provider.connection, payer: keypair, tab: 3 })
        }).then(() => {

          // investor: AIRDROP eqseedr token -> STAKETOKENMINT
          return stakeTokenMint
            .mintToken({ connection: provider.connection, payer: keypair, tab: 3 })
        })

        // push promise
        list.push(tx)

        // investor: GENERATE COMMIT AND SECRET 
        const passPhrase = "secret-" + i.toString()
        const secret = createHash('sha256')
          // will use session + passPhrase
          .update(keypair.publicKey.toBuffer())
          .update(passPhrase)
          .digest()

        const amount = new anchor.BN(100 + (random(i, i + target) * 100))
        const commitHash = createHash('sha256')
          .update(Buffer.from(amount.toString()))
          .update(keypair.publicKey.toBuffer())
          .update(secret)
          .digest()

        // store user / investor
        users.push({
          keypair,
          amount,
          secret,
          commitHash: new anchor.web3.PublicKey(commitHash),
          index: i + 1,
          bidTokenAccount: await getAssociatedTokenAddress(
            bidTokenMint.mint.keypair.publicKey,
            keypair.publicKey,
            true,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
          ),
          bidderStakeTokenAccount: await getAssociatedTokenAddress(
            stakeTokenMint.mint.keypair.publicKey,
            keypair.publicKey,
            true,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
          )
        })
      }

      await Promise.all(list)
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
            launchDate: new anchor.BN(1723613050),
            tokenAllocation: new anchor.BN(1_000_000_000 * 1_000_000),
          }
        })


      })

      it("Create Sealed Bid round", async () => {

        await script.createSessionSealedBidRound({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
        })

      })

      it("Create Commit Leader Board", async () => {
        await script.createCommitLeaderBoard({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
        })
      })

      it("Reallocate Commit Leader Board", async () => {
        await script.reallocateCommitLeaderBoard({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
        })
      })

      it("Create Token Stake Vault", async () => {
        await script.createTokenStakeVault({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          stakeTokenMint,
        })
      })

      it("Create Commit Bid Vault", async () => {
        await script.createCommitBidVault({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
          bidTokenMint,
        })

      })

      // should also mint the allocated tokens
      it("Create Vested Token Escrow", async () => {
        await script.createVestedTokenEscrow({
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

      it("Create Vested Config By Session", async () => {

        await script.createVestedConfig({
          connection: provider.connection,
          authority: tokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint,
        })
      })

      // it("Create Session Tick Bid Leader Board", async () => {
      //   await script.createSessionTickBidLeaderBoard({
      //     connection: provider.connection,
      //     authority: tokenMint.mintAuthority,
      //     program,
      //     web3: anchor.web3,
      //     tokenMint,
      //   })
      // })

      // it("Create Tick Bid Marketplace", async () => {
      //   await script.createSessionMarketplace({
      //     connection: provider.connection,
      //     authority: tokenMint.mintAuthority,
      //     program,
      //     web3: anchor.web3,
      //     tokenMint,
      //   })
      // })

    })

    describe("Interact with Sealed Bid System", () => {


      it("Submit Sealed Bid", async () => {

        // await script.submitSealedBid({
        //   connection: provider.connection,
        //   authority: users[0].keypair,
        //   program: program,
        //   web3: anchor.web3,
        //   tokenMint,
        //   stakeTokenMint,
        //   input: users[0],
        // })

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

        // shuffle users. but don't like using it in tests.
        shuffle(users)
      })

      it("Submit Unsealed Bid", async () => {

        // await script.submitUnsealedBid({
        //   connection: provider.connection,
        //   authority: users[0].keypair,
        //   program: program,
        //   tokenMint,
        //   stakeTokenMint,
        //   input: {
        //     ...users[0],
        //     amount: new anchor.BN(users[0].amount),
        //     secretAsPub: new anchor.web3.PublicKey(users[0].secret)
        //   },
        // })

        const fn = async (index) => {

          if (index == users.length) {
            return
          }

          await script.submitUnsealedBid({
            connection: provider.connection,
            authority: users[index].keypair,
            program: program,
            tokenMint,
            stakeTokenMint,
            input: {
              ...users[index],
              amount: new anchor.BN(users[index].amount),
              secretAsPub: new anchor.web3.PublicKey(users[index].secret)
            },
          })

          index++
          await fn(index)
          shuffle(users)

        }

        await fn(0)

        // const {
        //   commitLeaderBoard,
        // } = script.getAccounts({
        //   tokenMint,
        //   program,
        //   bidTokenMint,
        //   stakeTokenMint,
        // })

      })

      // need to refind the test. needs to iterate through all the 
      // submit unsealed bids
      // once there are 10 bids in queue, then filter
      // out the lowest bids so it doesn't trigger constraint
      // main thing is not to trigger constaint because that is not
      // what we are testing for.
      // so their can be multiple ways to go about that.
      it("Submit Commit Bid", async () => {

        // await script.submitCommitBid({
        //   connection: provider.connection,
        //   authority: users[2].keypair,
        //   // authority: users[5].keypair,

        //   program: program,
        //   tokenMint,
        //   stakeTokenMint,
        //   bidTokenMint,
        //   input: users[2],
        //   // input: users[5],


        // })

        const fn = async (index) => {

          if (index == users.length - 5) {
            return
          }

          await script.submitCommitBid({
            connection: provider.connection,
            authority: users[index].keypair,

            program: program,
            tokenMint,
            stakeTokenMint,
            bidTokenMint,
            input: users[index],
          })

          index++
          await fn(index)
        }

        await fn(0)

        // const {
        //   commitQueue,
        // } = script.getAccounts({
        //   tokenMint,
        //   program,
        //   bidTokenMint,
        //   stakeTokenMint,
        // })


      })

      // request unlock staked tokens
      // request unclaimed commit bid
    })

    // describe("Interact with Tick Bid System", () => {

    //   // before
    //   // create 15 more users to test the tick bid system

    //   // registerVestedAccount -> pre ix
    //   it("Session Registration", async () => {
    //     try {
    //       await script.sessionRegistration({
    //         connection: provider.connection,
    //         web3: anchor.web3,
    //         authority: users[0].keypair,
    //         program,
    //         tokenMint,
    //         bidTokenMint,
    //         input: {
    //           vestedIndex: 1,
    //           vestedOwner: users[0].keypair.publicKey
    //         }
    //       })
    //     } catch (err) {
    //       console.log(err)
    //     }

    //   })
    //   // openBid
    //   // executeBid

    //   // registerVestedAccount
    //   // process commit for round, opens the round
    //   // execute bid
    //   // delayed execute bid
    //   // need to excute many bids quickly to close the tick bid round and enter another tick bid round
    //   // need to execute all the rounds fast to close the session
    // })


    // describe("Process Errors", () => { })
  })





});
