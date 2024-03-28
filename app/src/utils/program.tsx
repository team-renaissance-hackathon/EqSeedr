import { AnchorProvider, BN, Program } from "@coral-xyz/anchor";
import { PublicKey, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import { 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  TOKEN_PROGRAM_ID} from "@solana/spl-token";

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
  const newSealedBidRound =  await PublicKey.findProgramAddressSync([
    session.toBuffer(),
    Buffer.from("sealed-bid-round")], 
    PROGRAM_ID);

  return newSealedBidRound;
};

/* Derive New Session Commit Leaderboard */
export const getNewSessionCommitLeaderboard = async (session) => {
  const newSessionCommitLeaderboard = await PublicKey.findProgramAddressSync([
    session.toBuffer(),
    Buffer.from("commit-leader-board")],
    PROGRAM_ID);

  return newSessionCommitLeaderboard;
}

/* Derive New Session Commit Queue */
export const getNewSessionCommitQueue = async (session) => {
  const newSessionCommitQueue = await PublicKey.findProgramAddressSync([
    session.toBuffer(),
    Buffer.from("commit-queue")],
    PROGRAM_ID);

  return newSessionCommitQueue;
}

/* TODO Derive New Sealed Bid Token Stake Account*/
// export const getNewSealedBidTokenStakeAccount = async (session) => {
//   const newSealedBidTokenStakeAccount = await PublicKey.findProgramAddressSync([
//   ],
//   PROGRAM_ID)

//   return newSealedBidTokenStakeAccount;
// }

/* Derive New Tick Bid Round */
export const getNewTickBidRound = async (session, sessionNextRound) => {
  
  const newTickBidRound = await PublicKey.findProgramAddressSync([
    Buffer.from(sessionNextRound),
    session.toBuffer(),
    Buffer.from("tick-bid-round")],
    PROGRAM_ID);

  return newTickBidRound;
}

/* Derive New Session Tick Bid Leaderboard */
export const getNewSessionTickBidLeaderboard = async (session) => {
  const newSessioinTickBidLeaderboard = await PublicKey.findProgramAddressSync([
    session.toBuffer(),
    Buffer.from("tick-bid-leader-board")],
    PROGRAM_ID);

  return newSessioinTickBidLeaderboard;    
}

/* Derive New Marketplace Positions */
export const getNewMarketplacePositions = async (session) => {
  const newMarketplacePosition = await PublicKey.findProgramAddressSync([
    session.toBuffer(),
    Buffer.from("session-marketplace")],
    PROGRAM_ID);

  return newMarketplacePosition;
}

/* Derive New Vested Config By Session */
export const getNewVestedConfigBySession = async (session) => {
  const newVestedConfigBySession = await PublicKey.findProgramAddressSync([
    session.toBuffer(),
    Buffer.from("vested-config")],
    PROGRAM_ID)

  return newVestedConfigBySession;
}



/* reference */

// export const getLotteryAddress = async (id) => {
//   return (
//     await PublicKey.findProgramAddress(
//       [Buffer.from(LOTTERY_SEED), new BN(id).toArrayLike(Buffer, "le", 4)],
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
