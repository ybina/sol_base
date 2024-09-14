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
  it("create token!", async () => {
    const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    const mintToken = anchor.web3.Keypair.generate();
    const mintPubkey = mintToken.publicKey;
    const [metadataPDA, _] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer()
      ],
      TOKEN_METADATA_PROGRAM_ID,
    )
    try {
      // const [authorityPda, authorityBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      //   [Buffer.from("authority_pda")],
      //   program.programId
      // );
      const [counterPda, counterBump] = await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("account_counter")],
        program.programId
      );
      const tx = await program.methods.createToken("pe17692 coin", "pe17692", "https://www.baidu.com/src").accounts({
        signer: signer,
        counter: counterPda,
      }).rpc({
        commitment: "confirmed",
        preflightCommitment: "processed",
        skipPreflight: true,
        maxRetries: 3,
      });
      console.log("transaction signature", tx);

    } catch(error) {
      console.log("error:", error)
    }
    
  });
});
