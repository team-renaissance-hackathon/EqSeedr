import { AnchorProvider, BN, Program } from "@coral-xyz/anchor";
import { PublicKey, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import { 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  TOKEN_PROGRAM_ID, 
  getAssociatedTokenAddressSync } from "@solana/spl-token";

import idl from "./idl.json";
import {
  EQSEEDR_IDL,
  PROGRAM_ID,
} from "./constants";

// Fetching our Program
export const getProgram = (connection, wallet) => {
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });

  const program = new Program(EQSEEDR_IDL, PROGRAM_ID, provider);

  console.log("Program ID:", PROGRAM_ID.toBase58());

  return program;
};

/* Deriving initialize accounts addresses */
export const getNewAuthority = async () => {
  return (
    await PublicKey.findProgramAddressSync([
      Buffer.from("authority")], 
      PROGRAM_ID)
  );
};

export const getNewIndexerStatus = async (newAuthorityPk) => {
  return (
    await PublicKey.findProgramAddressSync([
      Buffer.from("indexer-status"),
      newAuthorityPk[0].toBuffer()], 
      PROGRAM_ID)
  );
};

export const getNewActiveSessionIndexer = async (newAuthorityPk) => {
  return (
    await PublicKey.findProgramAddressSync([
      Buffer.from("active-session-indexer"),
      newAuthorityPk[0].toBuffer()], 
      PROGRAM_ID)
  );
};

export const getNewEnqueueSessionIndexer = async (newAuthorityPk) => {
  return (
    await PublicKey.findProgramAddressSync([
      Buffer.from("enqueue-session-indexer"),
      newAuthorityPk[0].toBuffer()], 
      PROGRAM_ID)
  );
};

export const getNewMarketPlaceMatchers = async (newAuthorityPk) => {
  return (
    await PublicKey.findProgramAddressSync([
      Buffer.from("marketplace-matchers"),
      newAuthorityPk[0].toBuffer()],
      PROGRAM_ID)
  );
};

export const getNewProgramMint = async (newAuthorityPk) => {
  return (
    await PublicKey.findProgramAddressSync([
      newAuthorityPk[0].toBuffer(),
      Buffer.from("token-mint")],
      PROGRAM_ID)
  );
};

// getAssociatedTokenAddressSync but manually
export const getNewAuthorityTokenAccount = async (programAuthority, programMint) => {
  return (
    await PublicKey.findProgramAddressSync([
      programAuthority[0].toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      programMint[0].toBuffer()],
      ASSOCIATED_TOKEN_PROGRAM_ID)
  );
};

/* Derive New Session address */ 
export const getNewSession = async (token_mint) => {
  return (
    await PublicKey.findProgramAddressSync([
      token_mint.toBuffer(),
      Buffer.from("session")],
      PROGRAM_ID)
  );
};

/* Derive new Sealed Bid round */
export const getNewSealedBidRound = async (session) => {
  return(
    await PublicKey.findProgramAddressSync([
      session.toBuffer(),
      Buffer.from("sealed-bid-round")], 
      PROGRAM_ID)
  );
};

// 

// export const getLotteryAddress = async (id) => {
//   return (
//     await PublicKey.findProgramAddress(
//       [Buffer.from(LOTTERY_SEED), new BN(id).toArrayLike(Buffer, "le", 4)],
//       PROGRAM_ID
//     )
//   )[0];
// };

// export const getTicketAddress = async (lotteryPk, id) => {
//   return (
//     await PublicKey.findProgramAddress(
//       [
//         Buffer.from(TICKET_SEED),
//         lotteryPk.toBuffer(),
//         new BN(id).toArrayLike(Buffer, "le", 4),
//       ],
//       PROGRAM_ID
//     )
//   )[0];
// };

// // Return the lastTicket ID and multiply the ticket price and convert LAMPORTS PER SOL and convert it to String
// export const getTotalPrize = (lottery) => {
//   return new BN(lottery.lastTicketId)
//     .mul(lottery.ticketPrice)
//     .div(new BN(LAMPORTS_PER_SOL))
//     .toString();
// };
