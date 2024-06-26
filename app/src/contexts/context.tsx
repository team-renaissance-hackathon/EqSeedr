import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { 
  Connection, 
  PublicKey, 
  LAMPORTS_PER_SOL, 
  clusterApiUrl,
  SystemProgram, 
  Transaction} from '@solana/web3.js';
import { useWallet, useConnection, useAnchorWallet } from "@solana/wallet-adapter-react";
import bs58 from "bs58";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID  } from "@solana/spl-token";

import {
  getNewActiveSessionIndexer,
  getNewAuthority,
  getNewEnqueueSessionIndexer,
  getNewIndexerStatus,
  getProgram,
  getNewSession,
  getNewMarketPlaceMatchers,
  getNewProgramMint,
  getNewAuthorityTokenAccount,
  getNewSealedBidRound,
  getNewSessionCommitLeaderboard,
  getNewSessionCommitQueue,
  getNewSessionTickBidLeaderboard,
  getNewMarketplacePositions,
  getNewVestedConfigBySession,
  getNewTickBidRound,
  getNewVestedAccountByOwner,
  getNewVestedAccountByIndex,
} from "../utils/program";

import { confirmTx, mockWallet } from "../utils/helper";
import toast from "react-hot-toast";
import { getMint } from "@solana/spl-token";
import { BN } from "@coral-xyz/anchor";
import { WalletKeypairError } from "@solana/wallet-adapter-base";

export const AppContext = createContext(null);

export const AppProvider = ({ children }) => {
  // Wallet state variables
  const [walletAddress, setwalletAddress] = useState("");
  const [walletBalance, setWalletBalance] = useState(0);
 
  /* Initializing the Program state variables*/
  const [indexerStatus, setIndexerStatus] = useState('');
  const [newAuthority, setNewAuthority] = useState('');
  const [activeSessionIndexer, setActiveSessionIndexer] = useState('');
  const [enqueueSessionIndexer, setEnqueueSessionIndexer] = useState('');
  const [marketplaceMatcher, setMarketplaceMatcher] = useState('');
  const [programMint, setProgramMint] = useState('');
  const [authorityTokenAccount, setAuthorityTokenAccount] = useState('');
  
  /* Session creation state variables */
  const [tokenMint, setTokenMint] = useState("");
  const [currentSession, setSession] = useState("");
  
  /* Session Sealed bid round creation state variables */
  const [sessionSealedBidRound, setSessionSealedBidRound] = useState("");

  /* Session Commit Leaderboard creation state variables */
  const [sessionCommitLeaderboard, setSessionCommitLeaderboard] = useState("");

  /* Session Commit Queue creation state variables */
  const [sessionCommitQueue, setSessionCommitQueue] = useState("");

  /* Session Tick Bid Leaderboard creation state variables */
  const [sessionTickBidLeaderBoard, setSessionTickBidLeaderboard] = useState("");

  /* Session Marketplace creation state variables */
  const [sessionMarketplace, setSessionMarketplace] = useState("");

  /* Vested Config by Session creation state variables */
  const [vestedConfigBySession, setVestedConfigBySession] = useState("");

  /* Session Registration creation state variables */
  const [vestedAccountByOwner, setVestedAccountByOwner] = useState("");
  const [vestedAccountByIndex, setVestedAccountByIndex] = useState([]);

  // Get provider
  const { connection } = useConnection();
  const wallet = useAnchorWallet();
  const program = useMemo(() => {
    if (connection) {
      return getProgram(connection, wallet ?? mockWallet());
    }
  }, [connection, wallet]);

  useEffect(() => {
    updateState()
  }, [program])

  const updateState = async () => {
    if(!program) return;

    try{
      if(!walletAddress) {
        //Get the wallet address
        setwalletAddress(new PublicKey(wallet.publicKey).toBase58());
      }
      // Get the balance of the connected wallet
      const walletBal = await connection.getBalance(new PublicKey(walletAddress));
      setWalletBalance( walletBal / LAMPORTS_PER_SOL);
        
    }catch(err){
      console.log(err.message);
    }
  }

  /* Calling of Smart Contract Instructions */

  /* Initialize instruction */ 
  const initLaunchPad = async () => {
    try{
      // Derive the New Authority address
      const newAuthorityAddress = await getNewAuthority();
      setNewAuthority(newAuthorityAddress[0].toBase58());
      console.log("New Authority: ", newAuthority);

      // Derive the New Indexer Status address
      const newIndexerStatusAddress = await getNewIndexerStatus(newAuthorityAddress);
      setIndexerStatus(newIndexerStatusAddress[0].toBase58())
      console.log("New Indexer Status: ", indexerStatus);

      // Derive the New Active Session Indexer address
      const newActiveSessionIndexerAddress = await getNewActiveSessionIndexer(newAuthorityAddress);
      setActiveSessionIndexer(newActiveSessionIndexerAddress[0].toBase58());
      console.log("New Active Session Indexer Status: ", activeSessionIndexer);
      
      // Derive the New Enqueue Session Indexer address
      const newEnqueueSessionIndexerAddress = await getNewEnqueueSessionIndexer(newAuthorityAddress);
      setEnqueueSessionIndexer(newEnqueueSessionIndexerAddress[0].toBase58());
      console.log("New Enqueue Session Indexer Status: ", enqueueSessionIndexer);

      // Derive the New Program Mint
      const newProgramMintAddress = await getNewProgramMint(newAuthorityAddress);
      setProgramMint(newProgramMintAddress[0].toBase58());
      console.log("Program Mint: ", programMint);

      // Derive the New Authority Token Account
      const newAuthorityTokenAccountAddress = await getNewAuthorityTokenAccount(newAuthorityAddress, newProgramMintAddress);
      setAuthorityTokenAccount(newAuthorityTokenAccountAddress[0].toBase58());
      console.log("New Authority Token Account: ", authorityTokenAccount);

      // Derive the New Marketplace Matcher address
      const newMarketPlaceMatcherAddress = await getNewMarketPlaceMatchers(newAuthorityAddress);
      setMarketplaceMatcher(newMarketPlaceMatcherAddress[0].toBase58());
      console.log("New Marketplace Matcher: ", marketplaceMatcher);

      // Invoking the initialize instruction on the smart contract
      // const txHash = await program.methods.initialize()
      // .accounts({ 
      //   authority: new PublicKey(walletAddress),
      //   newAuthority: newAuthorityAddress[0],
      //   newTokenMint: newProgramMintAddress[0],
      //   newAuthorityTokenAccount: newAuthorityTokenAccountAddress[0],
      //   newIndexerStatus: newIndexerStatusAddress[0],
      //   newActiveSessionIndexer: newActiveSessionIndexerAddress[0],
      //   newEnqueueSessionIndexer: newEnqueueSessionIndexerAddress[0],
      //   newMarketplaceMatchers: newMarketPlaceMatcherAddress[0],
      //   associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      //   tokenProgram: TOKEN_PROGRAM_ID,
      //   systemProgram: SystemProgram.programId
      // })
      // .rpc()

      // await confirmTx(txHash, connection);
      toast.success("Session states initialized!");
    }catch(err){
      console.log(err);
      toast.error(err.message);
    }
  }

  /* Create Session instruction */
  const createSession = async (sessionParams) => {
    try{
      // Get Mint Account Information
      const token_Mint = new PublicKey("5424Nqzfm4z7hahk4C3N5G3qDobt9d9s5kef2C2dNTs1");
      setTokenMint(token_Mint.toBase58());
      const mintInfo = await getMint(connection, token_Mint);
      const mint_decimals = mintInfo.decimals;
      setTokenMint(token_Mint.toBase58());

      // Format Token Allocatio to the mint decimals
      const tokenAllocation_decimal = sessionParams.tokenAllocation * (10**(mint_decimals));
      const tokenAllocation_BN = new BN(tokenAllocation_decimal);
      sessionParams.tokenAllocation = tokenAllocation_BN;

      // Get the value of newSession
      const newSession = await getNewSession(new PublicKey(tokenMint));
      setSession(newSession[0].toBase58());

      console.log("Token Name:",sessionParams.tokenName);
      console.log("Token Allocation:",sessionParams.tokenAllocation.toNumber());
      console.log("Launch Date:",sessionParams.launchDate.toNumber());
      
      console.log("New Session: ", newSession[0].toBase58());
      console.log("Authority: ",wallet.publicKey.toBase58());
      console.log("Indexer: ", indexerStatus);
      console.log("Token Mint: ", token_Mint.toBase58());

      // Invoking the createSession instruction on the smart contract
      // const txHash = await program.methods
      // .createSession(sessionParams)
      // .accounts({
      //   authority: wallet.publicKey,
      //   indexer: new PublicKey(indexerStatus),
      //   newSession: newSession[0],
      //   tokenMint: new PublicKey(tokenMint),
      // })
      // .rpc()

      // await confirmTx(txHash, connection);
      // console.log("Transaction: ", txHash);

      toast.success("Session created!")
    }catch(err){
      console.log(err);
      toast.error(err.message);
    }
  }

  // TODO SKIP FOR NOW
  /* Create Commit Token Account instruction */
  // const createCommitTokenAccount = async () => {
  //   try{
  //     const txHash = await program.methods.createCommitTokenAccount()
  //     .accounts({
  //       authority: wallet.publicKey,
  //       programAuthority: new PublicKey(newAuthority),
  //       newCommitTokenAccount: ,
  //       bidTokenMint: ,
  //     })
  //     .rpc()
  //   }
  // }

  /* Create Session Sealed Bid Round */
  const createSessionSealedBidRound = async () => {
    try{
      // Get the new Sealed Bid Round address
      const currentSessionPk = new PublicKey(currentSession);
      const newSealedBidRoundAddress = await getNewSealedBidRound(currentSessionPk);
      setSessionSealedBidRound(newSealedBidRoundAddress[0].toBase58());

      console.log("New Sealed Bid Round: ", newSealedBidRoundAddress[0].toBase58());

      // const txHash = await program.methods
      // .createSessionSealedBidRound()
      // .accounts({
      //   authority: wallet.publicKey,
      //   newSealedBidRound: newSealedBidRoundAddress[0],
      //   session: currentSessionPk,
      // })
      // .rpc()

      // await confirmTx(txHash, connection);
      console.log("Sealed Bid Round created successfully!")
      toast.success("Session Commit Leaderboard created successfully!");
    }catch(err){
      console.log(err);
      toast.error(err.message);
    }
  }

  /* Create Session Commit LeaderBoard */
  const createSessionCommitLeaderBoard = async () => {
    try{
      // Get the New Session Commit Leaderboard address
      const currentSessionPk = new PublicKey(currentSession);
      const newSessionCommitLeaderboardAddress = await getNewSessionCommitLeaderboard(currentSessionPk);
      setSessionCommitLeaderboard(newSessionCommitLeaderboardAddress[0].toBase58());
    
      // const txHash = await program.methods
      // .createSessionCommitLeaderBoard()
      // .accounts({
      //   authority: wallet.publicKey,
      //   newCommitLeaderBoard: newSessionCommitLeaderboardAddress[0],
      //   session: currentSessionPk,
      // })
      // .rpc()

      // await confirmTx(txHash, connection);
      console.log("Session Commit Leaderboard created successfully!");
      toast.success("Session Commit Leaderboard created successfully!");
    }catch(err){
      console.log(err);
      toast.error(err.message);
    }
  }

  /* Create Session Commit Queue */
  const createSessionCommitQueue = async () => {
    try{
      // Get the New Session Commit Queue address
      const currentSessionPk = new PublicKey(currentSession);
      const newSessionCommitQueueAddress = await getNewSessionCommitQueue(currentSessionPk);
      setSessionCommitQueue(newSessionCommitQueueAddress[0].toBase58());
    
      // const txHash = await program.methods
      // .createSessionCommitQueue()
      // .accounts({
      //   authority: wallet.publicKey,
      //   newCommitQueue: newSessionCommitQueueAddress[0],
      //   session: currentSessionPk,
      // })
      // .rpc()
      // await confirmTx(txHash, connection);

      console.log("Session Commit Queue created successfully!");
      toast.success("Session Commit Queue created successfully!");
    }catch(err){
      console.log(err);
      toast.error(err.message);
    }
  }

  // TODO -RELATED TO TOKEN ACCOUNT INSTRUCTION
  // /* Create Sealed Bid Token Stake Account */
  // const createSealedBidTokenStakeAccount = async () => {
  //   try{
  //     // Get the New Session Commit Queue address
  //     const currentSessionPk = new PublicKey(currentSession);
  //     const newSealedBidTokenStakeAccount = await getNewSessionCommitQueue(currentSessionPk);
  //     setSessionCommitLeaderboard(newSealedBidTokenStakeAccount[0].toBase58());
    
  //     // const txHash = await program.methods
  //     // .createSessionCommitQueue()
  //     // .accounts({
  //     //   authority: wallet.publicKey,
  //     //   newCommitQueue: newSessionCommitQueueAddress[0],
  //     //   session: currentSessionPk,
  //     // })
  //     // .rpc()
  //     // await confirmTx(txHash, connection);

  //     console.log("Session Commit Queue created successfully!");
  //     toast.success("Session Commit Queue created successfully!");
  //   }catch(err){
  //     console.log(err);
  //     toast.error(err.message);
  //   }
  // }

  /* Create 10 Tick Bid Round */ 
  const createTickBidRound = async () => {
    try{
      const currentSessionPk = new PublicKey(currentSession);
      const txHash = [];

      // Create 10 instance of create New Tick Bid Round
      // and store to transactions as a bundle
      for (let i = 1; i <= 10; i++){
        // Get the New Tick Bid Round address
        const newTickBidRoundAddress = await getNewTickBidRound(currentSessionPk, i.toString());
        setSessionCommitLeaderboard(newTickBidRoundAddress[0].toBase58());

        // If it's the 10th Tick Bid Round execuate and send the transaction
        if (i == 10){
          console.log(`Tick Bid Round [${i}]: `,newTickBidRoundAddress[0].toBase58());

          // Create a new transaction for the 10th round and perform pre instruction
          // txHash[i] = await program.methods
          // .createTickBidRound()
          // .preInstructions([
          //   txHash[1].instruction,
          //   txHash[2].instruction,
          //   txHash[3].instruction,
          //   txHash[4].instruction,
          //   txHash[5].instruction,
          //   txHash[6].instruction,
          //   txHash[7].instruction,
          //   txHash[8].instruction,
          //   txHash[9].instruction,
          // ])
          // .accounts({
          //   authority: wallet.publicKey,
          //   newTickBidRound: newTickBidRoundAddress[0],
          //   session: currentSessionPk
          // })
          // .rpc()

          // await confirmTx(txHash[i], connection);

        } else {
          console.log(`Tick Bid Round [${i}]: `,newTickBidRoundAddress[0].toBase58());

          // Create a new transaction for the current round
          txHash[i] = await program.methods
          .createTickBidRound()
          .accounts({
            authority: wallet.publicKey,
            newTickBidRound: newTickBidRoundAddress[0],
            session: currentSessionPk
          })
          .prepare()
        }
      }

      console.log("Tick Bid Round created successfully!");
      toast.success("Tick Bid Round created successfully!");
    }catch(err){
      console.log(err);
      toast.error(err.message);
    }
  }

  /* Create Session Tick Bid Leaderboard */
  const createSessionTickBidLeaderboard = async () => {
    try{
      // Get the New Session Tick Bid Leaderboard address
      const currentSessionPk = new PublicKey(currentSession);
      const newTickBidLeaderboardAddress = await getNewSessionTickBidLeaderboard(currentSessionPk);
      setSessionTickBidLeaderboard(newTickBidLeaderboardAddress[0].toBase58());
    
      // const txHash = await program.methods
      // .createSessionTickBidLeaderBoard()
      // .accounts({
      //   authority: wallet.publicKey,
      //   newTickBidLeaderBoard: newTickBidLeaderboardAddress[0],
      //   session: currentSessionPk,
      // })
      // .rpc()
      // await confirmTx(txHash, connection);

      console.log("Tick Bid Leaderboard created successfully!");
      toast.success("Tick Bid Leaderboard created successfully!");
    }catch(err){
      console.log(err);
      toast.error(err.message);
    }
  }

  /* Create Session Marketplace */
  const createSessionMarketplace = async () => {
    try{
      // Get the New Session Marketplace address
      const currentSessionPk = new PublicKey(currentSession);
      const newSessionMarketplaceAddress = await getNewMarketplacePositions(currentSessionPk);
      setSessionMarketplace(newSessionMarketplaceAddress[0].toBase58());
    
      // const txHash = await program.methods
      // .createSessionMarketplace()
      // .accounts({
      //   authority: wallet.publicKey,
      //   newMarketplacePositions: newSessionMarketplaceAddress[0],
      //   session: currentSessionPk,
      // })
      // .rpc()
      // await confirmTx(txHash, connection);

      console.log("Session Marketplace created successfully!");
      toast.success("Session Marketplace created successfully!");
    }catch(err){
      console.log(err);
      toast.error(err.message);
    }
  }

  /* Create Vested Config by Session */
  const createVestedConfigBySession = async () => {
    try{
      // References sessoin token mint address
      const token_mint = new PublicKey(tokenMint);

      // Get the Vested Config address
      const currentSessionPk = new PublicKey(currentSession);
      const newVestedConfigBySession = await getNewVestedConfigBySession(currentSessionPk);
      setVestedConfigBySession(newVestedConfigBySession[0].toBase58());
    
      // const txHash = await program.methods
      // .createVestedConfigBySession()
      // .accounts({
      //   authority: wallet.publicKey,
      //   newVestedConfig: newVestedConfigBySession[0],
      //   session: currentSessionPk,
      //   tokenMint: token_mint,
      // })
      // .rpc()
      // await confirmTx(txHash, connection);

      console.log("Vested Config created successfully!");
      toast.success("Vested Config created successfully!");
    }catch(err){
      console.log(err);
      toast.error(err.message);
    }
  }

  /* Create Session Registration */
  const createSessionRegistration = async () => {
    try{
      // References session token mint address
      const token_mint = new PublicKey(tokenMint);

      // Reference the Session address
      const currentSessionPk = new PublicKey(currentSession);

      // Get the New Vested Account By Owner address
      const newVestedAccountByOwnerAddress = await getNewVestedAccountByOwner(currentSessionPk, wallet.publicKey);
      setVestedAccountByOwner(newVestedAccountByOwnerAddress[0].toBase58());

      console.log("New Vested Account by Owner: ", newVestedAccountByOwnerAddress[0].toBase58());

      // Reference the Vested Config by Session address
      const vestedConfigBySessionPk = new PublicKey(vestedConfigBySession);
      
      // Fetch the index of vested config by session
      const vestedConfigData = await program.account.vestedConfigBySession.fetch(vestedConfigBySessionPk);
      const vestedConfigdIndex = (vestedConfigData.vestedIndex + 1);

      console.log("Vested Config Index: ", vestedConfigdIndex)

      // Get the New Vested Account By Index address
      const newVestedAccountByIndexAddress = await getNewVestedAccountByIndex(currentSessionPk, vestedConfigdIndex)
      const updatedVestedAccountByIndex = [...vestedAccountByIndex, newVestedAccountByIndexAddress[0].toBase58()];
      setVestedAccountByIndex(updatedVestedAccountByIndex);
      console.log("New Vested Account by Index Address: ", updatedVestedAccountByIndex);

      // Invoke the sessionRegistration instruction from the smart contract
      // const txHash = await program.methods.sessionRegistration()
      // .accounts({
      //   authority: wallet.publicKey,
      //   newVestedAccountByOwner: newVestedAccountByOwnerAddress[0],
      //   newVestedAccountByIndex: newVestedAccountByIndexAddress[0],
      //   vestedConfig: vestedConfigBySessionPk,
      //   session: currentSessionPk,
      //   tokenMint: token_mint,
      // })
      // .rpc()
      // await confirmTx(txHash, connection);

      toast.success("Session Registration complete!")
      console.log("Session Registration complete!")
    }catch(err){
      console.log(err)
      toast.error(err.message)
    }
  }


  return (
    <AppContext.Provider
      value={{
        /* Put functions/variables you want to bring 
          out of context from App in here */

        connected: wallet?.publicKey ? true : false,
        walletBalance: walletBalance,
        walletAddress : walletAddress,
        initLaunchPad,
        createSession,
        createSessionSealedBidRound,
        createSessionCommitLeaderBoard,
        createSessionCommitQueue,
        createSessionTickBidLeaderboard,
        createSessionMarketplace,
        createVestedConfigBySession,
        createTickBidRound,
        createSessionRegistration,
      }}
    >
      {children}
    </AppContext.Provider>
  );
};

export const useAppContext = () => {
  return useContext(AppContext);
};
