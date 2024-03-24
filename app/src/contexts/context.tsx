import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { 
  Connection, 
  PublicKey, 
  LAMPORTS_PER_SOL, 
  clusterApiUrl,
  SystemProgram } from '@solana/web3.js';
import { useWallet, useConnection, useAnchorWallet } from "@solana/wallet-adapter-react";
import bs58 from "bs58";

import {
  getNewActiveSessionIndexer,
  getNewAuthority,
  getNewEnqueueSessionIndexer,
  getNewIndexerStatus,
  getProgram,
} from "../utils/program";

import { confirmTx, mockWallet } from "../utils/helper";
import toast from "react-hot-toast";

export const AppContext = createContext(null);

export const AppProvider = ({ children }) => {
  // State variables
  const [walletAddress, setwalletAddress] = useState("");
  const [walletBalance, setWalletBalance] = useState(0);

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

  /* Call Solana Program Instructions */

  const initLaunchPad = async () => {
    try{
      const newAuthorityAddress = await getNewAuthority();
      console.log("New Authority: ", newAuthorityAddress[0].toBase58());

      const newIndexerStatusAddress = await getNewIndexerStatus(newAuthorityAddress);
      console.log("New Indexer Status: ", newIndexerStatusAddress[0].toBase58());

      const newActiveSessionIndexerAddress = await getNewActiveSessionIndexer(newAuthorityAddress);
      console.log("New Active Session Indexer Status: ", newActiveSessionIndexerAddress[0].toBase58());
      
      const newEnqueueSessionIndexerAddress = await getNewEnqueueSessionIndexer(newAuthorityAddress);
      console.log("New Enqueue Session Indexer Status: ", newEnqueueSessionIndexerAddress[0].toBase58());

      const txHash = await program.methods.initialize()
      .accounts({ 
        authority: new PublicKey(walletAddress),
        newAuthority: newAuthorityAddress[0],
        newIndexerStatus: newIndexerStatusAddress[0],
        newActiveSessionIndexer: newActiveSessionIndexerAddress[0],
        newEnqueueSessionIndexer: newEnqueueSessionIndexerAddress[0],
      })
      .rpc()

      await confirmTx(txHash, connection);
      toast.success("Session states initialized!");
    }catch(err){
      console.log(err.message);
      toast.error(err.message)
    }
  }

  return (
    <AppContext.Provider
      value={{
        // Put functions/variables you want to bring out of context to App in here
        connected: wallet?.publicKey ? true : false,
        walletBalance: walletBalance,
        walletAddress : walletAddress,
        initLaunchPad,
      }}
    >
      {children}
    </AppContext.Provider>
  );
};

export const useAppContext = () => {
  return useContext(AppContext);
};
