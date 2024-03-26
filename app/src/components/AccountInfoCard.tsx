import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import style from "../styles/AccountInfoCard.module.css";
import { useAppContext } from "../contexts/context";
import { shortenPk } from "../utils/helper";
import { Toaster } from 'react-hot-toast';
import { useEffect } from "react";
import CreateSessionCard from "./CreateSessionCard";
import Table from "./Table";

const AccountInfoCard = () => {

  // Getting the contexts
  const {
    connected,
    walletAddress,
    walletBalance,
    initLaunchPad,
  } = useAppContext();

  console.log(connected, "CONNECTION STATUS");

  // Display both wallet balance and wallet address
  console.log(`Wallet Balance: ${walletBalance} SOL`);
  console.log(`Wallet Address: ${shortenPk(walletAddress)}`);
 
  return (
    <div>
      {connected ? (
      <div>
        <div className={style.wrapper}>
          <Toaster />
          <div className={style.title}>
          Account Info
          </div>
  
          <div className={style.pot}>
          Wallet Address:
          </div>
  
          <div className={style.pot}> 
          {shortenPk(walletAddress)}
          </div>
  
          <div className={style.recentWinnerTitle}>
          Account Balance:
          </div>
          <div className={style.winner}>
          {walletBalance} SOL
          </div>
        </div>

        <div className={style.wrapper}>
            <div className={style.title}>Initialize</div>
            <button className={style.btn} onClick={initLaunchPad}>Initialize System</button>
        </div>

        <Table /> 
        <CreateSessionCard />
      </div>) : (
      <div className={style.wrapperCenter}>
        <div className={style.title}>
          Connect Account
        </div>
      </div>
    )}

    </div>
  );
};

export default AccountInfoCard;
