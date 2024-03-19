import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LaunchPad } from "../target/types/launch_pad";

import { script } from "../app/script";



describe("launch_pad", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.LaunchPad as Program<LaunchPad>;
  const keypair = anchor.web3.Keypair.generate()

  before(async () => {
    const tx = await provider.connection.requestAirdrop(
      keypair.publicKey,
      1000 * anchor.web3.LAMPORTS_PER_SOL
    )

    const latestBlockHash = await provider.connection.getLatestBlockhash()
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });
  })

  it("Is initialized!", async () => {
    await script.init({
      connection: provider.connection,
      authority: keypair,
      program,
      web3: anchor.web3
    })
  });
});
