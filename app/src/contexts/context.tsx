import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { 
  Connection, 
  PublicKey, 
  LAMPORTS_PER_SOL, 
  clusterApiUrl,
  SystemProgram } from '@solana/web3.js';
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
} from "../utils/program";

import { confirmTx, mockWallet } from "../utils/helper";
import toast from "react-hot-toast";
import { getMint } from "@solana/spl-token";
import { BN } from "@coral-xyz/anchor";
import { WalletKeypairError } from "@solana/wallet-adapter-base";

export const AppContext = createContext(null);

export const AppProvider = ({ children }) => {
  // State variables
  const [walletAddress, setwalletAddress] = useState("");
  const [walletBalance, setWalletBalance] = useState(0);
  const [indexerStatus, setIndexerStatus] = useState('');
  const [newAuthority, setNewAuthority] = useState('');
  const [activeSessionIndexer, setActiveSessionIndexer] = useState('');
  const [enqueueSessionIndexer, setEnqueueSessionIndexer] = useState('');
  const [tokenMint, setTokenMint] = useState("");
  

  /* For Initializing the Program */
  const [indexerStatus, setIndexerStatus] = useState('');
  const [newAuthority, setNewAuthority] = useState('');
  const [activeSessionIndexer, setActiveSessionIndexer] = useState('');
  const [enqueueSessionIndexer, setEnqueueSessionIndexer] = useState('');
  const [marketplaceMatcher, setMarketplaceMatcher] = useState('');
  const [programMint, setProgramMint] = useState('');
  const [authorityTokenAccount, setAuthorityTokenAccount] = useState('');

  const [tokenMint, setTokenMint] = useState("");

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

  // Initialize
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
      const newAuthorityTokenAccountAddress = await getNewAuthorityTokenAccount(wallet.publicKey, newProgramMintAddress);
      setAuthorityTokenAccount(newAuthorityTokenAccountAddress.toBase58());
      console.log("New Authority Token Account: ", authorityTokenAccount);

      // Derive the New Marketplace Matcher address
      const newMarketPlaceMatcherAddress = await getNewMarketPlaceMatchers(newAuthorityAddress);
      setMarketplaceMatcher(newMarketPlaceMatcherAddress[0].toBase58());
      console.log("New Marketplace Matcher: ", marketplaceMatcher);

      // Invoking the initialize instruction on the smart contract
      const txHash = await program.methods.initialize()
      .accounts({ 
        authority: new PublicKey(walletAddress),
        newAuthority: newAuthorityAddress[0],
        newTokenMint: newProgramMintAddress[0],
        newAuthorityTokenAccount: newAuthorityTokenAccountAddress.toBase58(),
        newIndexerStatus: newIndexerStatusAddress[0],
        newActiveSessionIndexer: newActiveSessionIndexerAddress[0],
        newEnqueueSessionIndexer: newEnqueueSessionIndexerAddress[0],
        newMarketplaceMatchers: newMarketPlaceMatcherAddress[0],
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      // .prepare()
      .rpc()

      // console.log(txHash);
      await confirmTx(txHash, connection);
      toast.success("Session states initialized!");
    }catch(err){
      console.log(err);
      toast.error(err.message);
    }
  }

  // Create Session
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

      console.log("Token Name:",sessionParams.tokenName);
      console.log("Token Allocation:",sessionParams.tokenAllocation.toNumber());
      console.log("Launch Date:",sessionParams.launchDate.toNumber());
      
      console.log("New Session: ", newSession[0].toBase58());
      console.log("Authority: ",wallet.publicKey.toBase58());
      console.log("Indexer: ", indexerStatus);
      console.log("Token Mint: ", token_Mint.toBase58());

      // Invoking the createSession instruction on the smart contract
      const txHash = await program.methods
      .createSession(sessionParams)
      .accounts({
        authority: wallet.publicKey,
        indexer: new PublicKey(indexerStatus),
        newSession: newSession[0],
        tokenMint: new PublicKey(tokenMint),
      })
      // .prepare()
      .rpc()

      // console.log(txHash)
      await confirmTx(txHash, connection);

      console.log("Transaction: ", txHash);

      toast.success("Session created!")
    }catch(err){
      console.log(err);
      toast.error(err.message);
    }
  }

  // 

  return (
    <AppContext.Provider
      value={{
        // Put functions/variables you want to bring out of context to App in here
        connected: wallet?.publicKey ? true : false,
        walletBalance: walletBalance,
        walletAddress : walletAddress,
        initLaunchPad,
        createSession,
      }}
    >
      {children}
    </AppContext.Provider>
  );
};

export const useAppContext = () => {
  return useContext(AppContext);
};
