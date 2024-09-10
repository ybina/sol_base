import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Test4 } from "../target/types/test4";
//import { TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
//import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("test4", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Test4 as Program<Test4>;
  const provider = anchor.AnchorProvider.env();
  const signer = provider.wallet.publicKey;
  it("create token!", async () => {
    const mintToken = anchor.web3.Keypair.generate()
    const tokenAccount = anchor.utils.token.associatedAddress({
      mint: mintToken.publicKey,
      owner: provider.wallet.publicKey,
    });
    //const associateTokenProgram = new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")
    //const tokenAccount =anchor.utils.token.associatedAddress({mint:mintToken.publicKey,owner:provider.publicKey})
    try {
      const tx = await program.methods.createToken(9, new anchor.BN(888888000000000)).accounts({
        signer: signer,
        mintToken: mintToken.publicKey,
        tokenAccount: tokenAccount,
      }).signers([mintToken]).rpc({
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
