import style from "../styles/AccountInfoCard.module.css";
import { useAppContext } from "../contexts/context";

const CreateSessionCard = () => {

    const {
        createSessionSealedBidRound,
        createSessionCommitLeaderBoard,
        createSessionCommitQueue,
        createSessionTickBidLeaderboard,
        createSessionMarketplace,
        createVestedConfigBySession,
        createTickBidRound,
        createSessionRegistration,
        
    } = useAppContext();

    
    return(
        <div className={style.wrapper}>
            <button className={style.btn} onClick={createSessionSealedBidRound}>Create Sealed Bid Round</button>
            <button className={style.btn} onClick={createSessionCommitLeaderBoard}>Create Session Commit Leaderboard</button>
            <button className={style.btn} onClick={createSessionCommitQueue}>Create Session Commit Queue</button>
            <button className={style.btn} >Create Sealed Bid Token Stake Account</button>
            <button 
                className={style.btn} 
                onClick={createTickBidRound}>
                    Create Tick Bid Round
            </button>
            <button 
                className={style.btn} 
                onClick={createSessionTickBidLeaderboard}>
                    Create Session Tick Bid Leaderboard
            </button>
            <button 
                className={style.btn} 
                onClick={createSessionMarketplace}>
                    Create Session Marketplace
            </button>
            <button 
                className={style.btn} 
                onClick={createVestedConfigBySession}>
                    Create Vested Config
            </button>
            <button 
                className={style.btn} 
                onClick={createSessionRegistration}>
                    Session Registration
            </button>
        </div>
             
    );
}

export default CreateSessionCard;