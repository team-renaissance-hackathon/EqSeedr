import { PublicKey } from "@solana/web3.js";
import idl from "./idl.json";

export const PROGRAM_ID = new PublicKey(idl.metadata.address);
export const EQSEEDR_IDL = JSON.parse(JSON.stringify(idl));
