import { AnchorProvider, BN, Program } from "@coral-xyz/anchor";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";

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

/* Initializing new Session Accounts */
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
