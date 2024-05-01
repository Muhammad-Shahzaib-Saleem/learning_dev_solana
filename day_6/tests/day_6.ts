import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day6 } from "../target/types/day_6";
import { BN } from "bn.js";

describe("day_6", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day6 as Program<Day6>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize("Shahzaib", new anchor.BN(22)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is age checker!", async () => {
    // Add your test here.
    const tx = await program.methods.ageChecker(new anchor.BN(17)).rpc();
    console.log("Your transaction signature", tx);
  });
});
