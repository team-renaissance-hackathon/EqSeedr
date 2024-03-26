import style from "../styles/AccountInfoCard.module.css";
import { useAppContext } from "../contexts/context";
import { shortenPk } from "../utils/helper";
import { Toaster } from 'react-hot-toast';
import { useState } from "react";
import { BN } from "@coral-xyz/anchor";

const CreateSessionCard = () => {

    const [tokenName, setTokenName] = useState('');
    const [launchDate, setLaunchDate] = useState('');
    const [tokenAllocation, setTokenAllocation] = useState('');
    const [sessionParams, setSessionParams] = useState(null);

    const {
        createSession,
    } = useAppContext();

    const handleSubmit = (event) => {
        event.preventDefault();

       
        // Convert launchdate to unix_timestamp format
        const launchDateData = new BN(Date.parse(new Date(launchDate).toLocaleDateString()));

        // Create a sessionParams Object to store in sessionParams
        const sessionParamsData = {
            tokenName,
            launchDate: launchDateData,
            tokenAllocation
        }

        setSessionParams(sessionParamsData);

        //Form submission logic
        createSession(sessionParams);

        console.log("Form submitted!");
    }

   
    
    return(
        <div className={style.wrapper}>
            <div className={style.title}>
                Create Session
            </div>

            <div>
                <form onSubmit={handleSubmit} className={style.form}>
                    <div className={style.formGroup}>
                    <label htmlFor="tokenName">Token Name:</label>
                    <input
                        type="text"
                        id="tokenName"
                        value={tokenName}
                        onChange={(e) => setTokenName(e.target.value)}
                        required
                    />
                    </div>

                    <div className={style.formGroup}>
                    <label htmlFor="launchDate">Launch Date:</label>
                    <input
                        type="date"
                        id="launchDate"
                        value={launchDate}
                        onChange={(e) => setLaunchDate(e.target.value)}
                        required
                    />
                    </div>

                    <div className={style.formGroup}>
                    <label htmlFor="tokenAllocation">Token Allocation:</label>
                    <input
                        type="number"
                        id="tokenAllocation"
                        value={tokenAllocation}
                        onChange={(e) => setTokenAllocation(e.target.value)}
                        required
                    />
                    </div>

                    <button className={style.btn} type="submit">Create</button>
                </form>
            </div>
        </div>
             
    );
}

export default CreateSessionCard;