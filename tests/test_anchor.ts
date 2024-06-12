import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TestAnchor } from "../target/types/test_anchor";
import { LAMPORTS_PER_SOL } from '@solana/web3.js';

describe("test_anchor", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TestAnchor as Program<TestAnchor>;

  it("Is initialized!", async () => {
    // Add your test here.
      let signer: anchor.web3.Keypair = anchor.web3.Keypair.generate();

      const user = anchor.web3.Keypair.generate();
      let token_airdrop = await provider.connection.requestAirdrop(user.publicKey,
        10 * LAMPORTS_PER_SOL);
    
      const latestBlockHash = await provider.connection.getLatestBlockhash();
    
      await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: token_airdrop,
      });


      const tx = await program.methods.initialize().accounts({
        systemProgram: anchor.web3.SystemProgram.programId,
        signer: signer.publicKey,
        user: user.publicKey,
      }).signers([signer, user]).rpc();
      console.log("Your transaction signature", tx);

    const tx2 = await program.methods.update(new anchor.BN(1), new anchor.BN(4)).accounts({
      signer: signer.publicKey,
      user: user.publicKey,
    }).signers([signer]).rpc();
    console.log("Your transaction signature", tx2);
    
  });
});
