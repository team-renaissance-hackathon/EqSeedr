import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import dynamic from 'next/dynamic';
import style from "../styles/Header.module.css";

const Header = () => {

  const ButtonWrapper = dynamic (() =>
    import('@solana/wallet-adapter-react-ui').then((mod) => mod.WalletMultiButton)
  );

  return (
    <div className={style.wrapper}>
      <div className={style.title}>EqSeedr</div>
      <ButtonWrapper />
    </div>
  );
};

export default Header;
