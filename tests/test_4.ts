import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Test4 } from "../target/types/test4";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { ComputeBudgetProgram } from "@solana/web3.js";
import { BN } from "bn.js";
//import { TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
//import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("test4", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Test4 as Program<Test4>;
  const provider = anchor.AnchorProvider.env();
  const signer = provider.wallet.publicKey;

  // it("initialize", async ()=> {
  //   try {
  //     console.log("start call initialize")
  //     const tx = await program.methods.initProgram().accounts({
  //       signer: provider.wallet.publicKey,
  //     }).rpc({
  //       commitment: "confirmed",
  //       preflightCommitment: "processed",
  //       skipPreflight: true,
  //       maxRetries: 3,
  //     });
  //     console.log("initialize tx:", tx)
  //   } catch(error) {
  //     console.log("err:", error)
  //   }
  // });


  // it("create token!", async () => {
  //   // set maxmum CU
  //   const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({
  //     units: 500_000, 
  //   });
  //   try {
  //     const [counterPda, counterBump] = await anchor.web3.PublicKey.findProgramAddressSync(
  //       [Buffer.from("account_counter")],
  //       program.programId
  //     );
  //     const tx = await program.methods.createToken(
  //       "pd coin", "pdc", "https://www.google.com", 0.15).accounts({
  //       signer: signer,
  //       counter: counterPda,
  //     }).preInstructions([computeBudgetIx]).rpc({
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

  // it("buy test", async () => {

  //   const tokenPdaAddress = "3pYZUZb98kNvtpDZ2amBCZnS16Y3uDgz9SfT1gh3jDgp"
  //   const tokenPda = new anchor.web3.PublicKey(tokenPdaAddress);
  //   const mintPdaAddress = "CTkEMpJfZYo65HiXnYhhcTPbwR2yyQnRg4DMdPFCXvRT"
  //   const mintPda = new anchor.web3.PublicKey(mintPdaAddress)
  //   try {
  //     const tx = await program.methods.buy(0.1).accounts({
  //       signer: signer,
  //       tokenPda: tokenPda,
  //       mintPda: mintPda,
  //     }).rpc({
  //       commitment: "confirmed",
  //       preflightCommitment: "processed",
  //       skipPreflight: true,
  //       maxRetries: 3,
  //     });
  //     console.log("buy tx:", tx);

  //   } catch(error) {
  //     console.log("buy test failed:", error)

  //   }
    
  // })

  it("sell test", async () => {

    const tokenPdaAddress = "3pYZUZb98kNvtpDZ2amBCZnS16Y3uDgz9SfT1gh3jDgp"
    const tokenPda = new anchor.web3.PublicKey(tokenPdaAddress);
    const mintPdaAddress = "CTkEMpJfZYo65HiXnYhhcTPbwR2yyQnRg4DMdPFCXvRT"
    const mintPda = new anchor.web3.PublicKey(mintPdaAddress)
    try {
      const tx = await program.methods.sell(0.12).accounts({
        signer: signer,
        tokenPda: tokenPda,
        mintPda: mintPda,
      }).rpc({
        commitment: "confirmed",
        preflightCommitment: "processed",
        skipPreflight: true,
        maxRetries: 3,
      });
      console.log("sell tx:", tx);

    } catch(error) {
      console.log("sell test failed:", error)

    }
    
  })
});
