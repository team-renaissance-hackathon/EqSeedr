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
import { SYSTEM_PROGRAM_ID, program } from "@coral-xyz/anchor/dist/cjs/native/system";


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



class Token {
  // mint: anchor.web3.Keypair | anchor.web3.PublicKey;
  // mint: anchor.web3.Keypair;
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

    // console.log("\t".repeat(tab), "TOKEN MINT CREATED")

    await this.mintToken({ connection, payer, tab })
  }

  createTokenAccount = async ({ connection, payer, tab }) => {

    // console.log("\t".repeat(tab), "CREATE TOKEN ACCOUNT")

    const tokenAccount = await getAssociatedTokenAddress(
      this.mint.publicKey,
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
        this.mint.publicKey,
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
  const tokens = {

    // accepted tokens to bid in auction / stable coins
    bidTokenMint: [{ name: "USDC", token: new Token() }],

    // launchProjectTokenMint
    ventureTokenMint: [{ creator: anchor.web3.Keypair.generate(), token: new Token() }],

    // stakeTokenMint
    // programTokenMint: new Token(),
    programTokenMint: [],
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

      const [programTokenVault] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          programAuthority.toBuffer(),
          Buffer.from("program-token-vault")
        ],
        program.programId
      )

      const token = {
        mint: programTokenMint,
        mintAuthority: programAuthority,
        programTokenVault: programTokenVault,

        mintTokens: async ({
          signer,
          receipent,
          amount
        }) => {

          // console.log(programAuthority, programTokenMint, programTokenVault)
          // console.log(signer, receipent, amount)
          const tx = await program.methods
            .mintTokens(amount)
            .accounts({
              signer: signer.publicKey,
              programAuthority: programAuthority,
              receipent: receipent,
              tokenProgram: TOKEN_PROGRAM_ID,
            })
            .signers([signer])
            .rpc();

          const latestBlockHash = await provider.connection.getLatestBlockhash()
          await provider.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: tx,
          });

          // console.log("pass")
        },

        transfer: () => {

        }
      }

      tokens.programTokenMint.push(token)

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

    {
      await tokens.ventureTokenMint[0].token.createMint(
        provider.connection,
        keypair, undefined, undefined
      )
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

    // I think I got the mints mixed up...
    let stakeTokenMint = tokens.programTokenMint[0]
    let bidTokenMint = tokens.bidTokenMint.find(token => token.name === "USDC").token
    const ventureTokenMint = tokens.ventureTokenMint[0].token

    before(async () => {

      {
        const tx = await provider.connection.requestAirdrop(
          ventureTokenMint.mintAuthority.publicKey,
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

    // const target = 15
    const target = 10


    const users = []

    before(async () => {

      stakeTokenMint = tokens.programTokenMint[0]

      const list = []

      function random(min, max) { // min and max included 
        return Math.floor(Math.random() * (max - min + 1) + min)
      }

      for (let i = 0; i < target; i++) {

        // investor
        const keypair = anchor.web3.Keypair.generate();
        // const ata = getAssociatedTokenAddressSync(
        //   stakeTokenMint.mint,
        //   keypair.publicKey,
        // )

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
        }).then(async () => {

          return await getOrCreateAssociatedTokenAccount(
            provider.connection,
            keypair,
            stakeTokenMint.mint,
            keypair.publicKey
          )
        }).then((ata) => {

          // investor: AIRDROP eqseedr token -> STAKETOKENMINT
          return stakeTokenMint
            .mintTokens({ signer: keypair, receipent: ata.address, amount: new anchor.BN(1000 * LAMPORTS_PER_SOL) })
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

        const amount = new anchor.BN(100 + (random(i, i + target) * 100 + list.length))
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
            bidTokenMint.mint.publicKey,
            keypair.publicKey,
            true,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
          ),
          bidderStakeTokenAccount: await getAssociatedTokenAddress(
            stakeTokenMint.mint,
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
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
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
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
        })

      })

      it("Create Commit Leader Board", async () => {
        await script.createCommitLeaderBoard({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
        })
      })

      it("Reallocate Commit Leader Board", async () => {
        await script.reallocateCommitLeaderBoard({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
        })
      })

      it("Create Token Stake Vault", async () => {
        await script.createTokenStakeVault({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          stakeTokenMint,
        })
      })

      it("Create Commit Bid Vault", async () => {
        await script.createCommitBidVault({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          bidTokenMint,
        })

      })

      it("Create Venture Token Escrow", async () => {
        await script.createVentureTokenEscrow({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          bidTokenMint: bidTokenMint,
        })

      })

      // should also mint the allocated tokens
      it("Create Vested Token Escrow", async () => {
        await script.createVestedTokenEscrow({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
        })
      })

      it("Create Session Commit Queue", async () => {

        await script.createSessionCommitQueue({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
        })

      })

      it("Create Tick Bid Rounds", async () => {

        await script.createTickBidRound({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          bidTokenMint,
          roundIndex: 1,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          bidTokenMint,
          roundIndex: 2,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          bidTokenMint,
          roundIndex: 3,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          bidTokenMint,
          roundIndex: 4,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          bidTokenMint,
          roundIndex: 5,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          bidTokenMint,
          roundIndex: 6,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          bidTokenMint,
          roundIndex: 7,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          bidTokenMint,
          roundIndex: 8,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          bidTokenMint,
          roundIndex: 9,
        })

        await script.createTickBidRound({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
          bidTokenMint,
          roundIndex: 10,
        })

      })

      describe("Create Tick Bid Leader Board", () => {
        it("Trasnfer Rent Zero Copy", async () => {

          const [session] = anchor.web3.PublicKey.findProgramAddressSync(
            [
              ventureTokenMint.mint.publicKey.toBuffer(),
              Buffer.from("session"),
            ],
            program.programId
          )

          const [leaderBoard] = anchor.web3.PublicKey.findProgramAddressSync(
            [
              session.toBuffer(),
              Buffer.from("tick-bid-leader-board"),
            ],
            program.programId
          )

          console.log(leaderBoard)

          const tx = await program.methods
            .transferRentZeroCopy()
            .accounts({
              payer: keypair.publicKey,
              session,
            })
            .signers([keypair])
            .rpc()

          console.log("TX:", tx)

          const latestBlockHash = await provider.connection.getLatestBlockhash()
          await provider.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: tx,
          });
        })

        it("Assign Zero Copy", async () => {

          const [session] = anchor.web3.PublicKey.findProgramAddressSync(
            [
              ventureTokenMint.mint.publicKey.toBuffer(),
              Buffer.from("session"),
            ],
            program.programId
          )

          const tx = await program.methods
            .assignZeroCopy()
            .accounts({
              payer: keypair.publicKey,
              session,
            })
            .signers([keypair])
            .rpc()

          console.log("TX:", tx)

          const latestBlockHash = await provider.connection.getLatestBlockhash()
          await provider.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: tx,
          });
        })

        it("Reallocate Zero Copy", async () => {

          const [session] = anchor.web3.PublicKey.findProgramAddressSync(
            [
              ventureTokenMint.mint.publicKey.toBuffer(),
              Buffer.from("session"),
            ],
            program.programId
          )

          const tx = await program.methods
            .reallocZeroCopy()
            .accounts({
              payer: keypair.publicKey,
              session,
            })
            .signers([keypair])
            .rpc()

          const latestBlockHash = await provider.connection.getLatestBlockhash()
          await provider.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: tx,
          });
        })

        it("Initialize Zero Copy", async () => {

          const [session] = anchor.web3.PublicKey.findProgramAddressSync(
            [
              ventureTokenMint.mint.publicKey.toBuffer(),
              Buffer.from("session"),
            ],
            program.programId
          )

          const tx = await program.methods
            .initializeZeroCopy()
            .accounts({
              payer: keypair.publicKey,
              session,
            })
            .signers([keypair])
            .rpc()

          console.log("TX:", tx)

          const latestBlockHash = await provider.connection.getLatestBlockhash()
          await provider.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: tx,
          });
        })
      })

      it("Create Vested Config By Session", async () => {

        await script.createVestedConfig({
          connection: provider.connection,
          authority: ventureTokenMint.mintAuthority,
          program,
          web3: anchor.web3,
          tokenMint: ventureTokenMint,
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

        const fn = async (index) => {

          if (index == users.length) {
            return
          }

          await script.submitSealedBid({
            connection: provider.connection,
            authority: users[index].keypair,
            program: program,
            web3: anchor.web3,
            tokenMint: ventureTokenMint,
            stakeTokenMint,
            input: users[index],
          })

          index++
          await fn(index)
        }

        await fn(0)

        // shuffle users. but don't like using it in tests.
        // shuffle(users)
      })

      it("Submit Unsealed Bid", async () => {

        const fn = async (index) => {

          if (index == users.length) {
            return
          }

          await script.submitUnsealedBid({
            connection: provider.connection,
            authority: users[index].keypair,
            program: program,
            tokenMint: ventureTokenMint,
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

          // if (index == users.length - 5) {
          //   return
          // }

          if (index == users.length) {
            return
          }

          await script.submitCommitBid({
            connection: provider.connection,
            authority: users[index].keypair,

            program: program,
            tokenMint: ventureTokenMint,
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

    describe("Interact with Tick Bid System", () => {

      const commitQueue = []

      before("", async () => {
        users.forEach(user => {
          commitQueue.push(user)
        })

        commitQueue.sort((a, b) => b.amount - a.amount)

        users.sort((a, b) => a.index - b.index)


        // console.log("commit queue", commitQueue)
        // console.log("users", users)

      })

      // before
      // create 15 more users to test the tick bid system

      // registerVestedAccount -> pre ix
      it("Session Registration", async () => {

        const fn = async (index) => {

          if (index == users.length) {
            return
          }

          const user = users[index]

          await script.sessionRegistration({
            connection: provider.connection,
            web3: anchor.web3,
            authority: user.keypair,
            program,
            tokenMint: ventureTokenMint,
            bidTokenMint,
            input: {
              vestedIndex: user.index,
              vestedOwner: user.keypair.publicKey,
            }
          })

          index++
          await fn(index)
        }

        await fn(0)

      })

      it("Open Bid", async () => {
        console.log("OPEN BID:: ", commitQueue[0])
        console.log("OPEN BID:: ", commitQueue[0].keypair.publicKey)



        const fn = async (index) => {



          if (index == 10) {
            return
          }

          const user = commitQueue[index]

          await await script.openBid({
            connection: provider.connection,
            web3: anchor.web3,
            authority: user.keypair,
            program,
            tokenMint: ventureTokenMint,
            bidTokenMint,
            input: {
              vestedIndex: user.index,
              vestedOwner: user.keypair.publicKey
            }
          })

          index++
          await fn(index)
        }

        await fn(0)

        // await script.openBid({
        //   connection: provider.connection,
        //   web3: anchor.web3,
        //   authority: commitQueue[0].keypair,
        //   program,
        //   tokenMint: ventureTokenMint,
        //   bidTokenMint,
        //   input: {
        //     vestedIndex: commitQueue[0].index,
        //     vestedOwner: commitQueue[0].keypair.publicKey
        //   }
        // })



      })

      it("Execute Bid", async () => {

        function wait(milliseconds) {
          return new Promise(resolve => {
            setTimeout(resolve, milliseconds);
          });
        }
        const user = users[0]

        await wait(1000 * 10 + 10)

        await script.executeBid({
          connection: provider.connection,
          web3: anchor.web3,
          authority: user.keypair,
          program,
          tokenMint: ventureTokenMint,
          bidTokenMint,
          input: {
            vestedIndex: user.index,
            vestedOwner: user.keypair.publicKey,
            tokenAccount: user.bidTokenAccount,
          }
        })

        await wait(1000 * 60 * (2 + 3 + 4 + 5) + 1000)

        await script.executeBid({
          connection: provider.connection,
          web3: anchor.web3,
          authority: user.keypair,
          program,
          tokenMint: ventureTokenMint,
          bidTokenMint,
          input: {
            vestedIndex: user.index,
            vestedOwner: user.keypair.publicKey,
            tokenAccount: user.bidTokenAccount,
          }
        })

      })

      // registerVestedAccount
      // process commit for round, opens the round
      // execute bid
      // delayed execute bid
      // need to excute many bids quickly to close the tick bid round and enter another tick bid round
      // need to execute all the rounds fast to close the session
    })


    // describe("Process Errors", () => { })
  })





});
