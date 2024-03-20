import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LaunchPad } from "../target/types/launch_pad";

import {
  createInitializeMintInstruction,
  getMinimumBalanceForRentExemptMint,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token"

import { script } from "../app/script";


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
    payer: anchor.web3.Keypair
  ) => {

    this.mint = anchor.web3.Keypair.generate()
    this.mintAuthority = anchor.web3.Keypair.generate()
    this.freezeAuthority = anchor.web3.Keypair.generate()
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

    console.log("TOKEN MINT CREATED")
  }
}


describe("launch_pad", () => {

  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.LaunchPad as Program<LaunchPad>;
  const keypair = anchor.web3.Keypair.generate()
  const tokenMint = new Token()

  before(async () => {
    {
      const tx = await provider.connection.requestAirdrop(
        keypair.publicKey,
        1000 * anchor.web3.LAMPORTS_PER_SOL
      )

      const latestBlockHash = await provider.connection.getLatestBlockhash()
      await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
      });
    }

    {

      await tokenMint.createMint(
        provider.connection,
        keypair
      )
      const tx = await provider.connection.requestAirdrop(
        tokenMint.mintAuthority.publicKey,
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

  it("Is initialized!", async () => {

    await script.init({
      connection: provider.connection,
      authority: keypair,
      program,
      web3: anchor.web3
    })

  });

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

    // it("Create Session Sealed Bid round", async () => {

    //   await script.createSessionSealedBidRound({
    //     connection: provider.connection,
    //     authority: tokenMint.mintAuthority,
    //     program,
    //     web3: anchor.web3,
    //     tokenMint,
    //   })

    // })


  })





});
