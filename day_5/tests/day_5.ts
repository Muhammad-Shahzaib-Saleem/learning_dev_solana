import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day5 } from "../target/types/day_5";

import fs from "fs";
let idl = JSON.parse(fs.readFileSync('target/idl/day_5.json', 'utf-8'));

describe("day_5", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const programID = "JBaKPeGdDnddc1KyopkKxYN8i7Pw7qfkm2kHWQfRSzW4";

  const program = new Program(idl, programID, anchor.getProvider());
  // anchor.workspace.Day5 as Program<Day5>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
