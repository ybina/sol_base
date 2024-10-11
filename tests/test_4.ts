import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Test4 } from "../target/types/test4";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
//import { TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
//import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("test4", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Test4 as Program<Test4>;
  const provider = anchor.AnchorProvider.env();
  const signer = provider.wallet.publicKey;
  it("initialize", async ()=> {
    try {
      console.log("start call initialize")
      const tx = await program.methods.initProgram().accounts({
        signer: provider.wallet.publicKey,
      }).rpc({
        commitment: "confirmed",
        preflightCommitment: "processed",
        skipPreflight: true,
        maxRetries: 3,
      });
      console.log("initialize tx:", tx)
    } catch(error) {
      console.log("err:", error)
    }
  });
  // it("create token!", async () => {
    
  //   try {
  //     const [counterPda, counterBump] = await anchor.web3.PublicKey.findProgramAddressSync(
  //       [Buffer.from("account_counter")],
  //       program.programId
  //     );
  //     const tx = await program.methods.createToken(
  //       "pd coin", "pdc", "https://www.google.com").accounts({
  //       signer: signer,
  //       counter: counterPda,
  //     }).rpc({
  //       commitment: "confirmed",
  //       preflightCommitment: "processed",
  //       skipPreflight: true,
  //       maxRetries: 3,
  //     });
  //     console.log("transaction signature", tx);

  //   } catch(error) {
  //     console.log("error:", error)
  //   }
  // });
});
