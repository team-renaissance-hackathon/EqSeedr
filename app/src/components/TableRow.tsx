import style from "../styles/TableRow.module.css";
import { shortenPk } from "../utils/helper";

const TableRow = ({
  lotteryId,
  winnerAddress = "4koeNJ39zejjuCyVQdZmzsx28CfJoarrv4vmsuHjFSB6",
}) => {
  return (
    <tr className={style.wrapper}>
      <td>{lotteryId}</td>
      <td>{shortenPk(winnerAddress)}</td>
    </tr>
  );
};

export default TableRow;
