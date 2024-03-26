import style from "../styles/Table.module.css";
import TableRow from "./TableRow";

import { PublicKey } from '@solana/web3.js';

const Table = () => {
  const tableRows = [
    { lotteryId: 0, winnerAddress: new PublicKey("11111111111111111111111111111111"),  },
    { lotteryId: 1, winnerAddress: new PublicKey("11111111111111111111111111111111"),  },
    { lotteryId: 2, winnerAddress: new PublicKey("11111111111111111111111111111111"),  },
  ]

  return (
    <div> 
        <table className = {style.wrapper}>
          <tbody>
            <tr className = {style.tableHeader}> 
              <th>Accounts</th>
              <th>Address</th>
            </tr>

            {tableRows.map((rowData, index) =>(
              <TableRow key={index} {...rowData} />
            ))}
          </tbody>
        </table>
    </div>
  );
};

export default Table;
