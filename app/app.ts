import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LaunchPad } from "../target/types/launch_pad";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.LaunchPad as Program<LaunchPad>;