import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Lendingpr } from "../target/types/lendingpr";

describe("lendingpr", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Lendingpr as Program<Lendingpr>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
