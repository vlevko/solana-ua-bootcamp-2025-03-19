import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Favorites } from "../target/types/favorites";
import { airdropIfRequired, getCustomErrorMessage } from "@solana-developers/helpers";
import { expect, describe, test } from '@jest/globals';
import { systemProgramErrors } from "./system-program-errors";

describe("favorites", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("Writes our favorites to the blockchain", async () => {
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
      anchor.getProvider().connection,
      user.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    // Here's what we want to write to the blockchain
    const favoriteNumber = new anchor.BN(23);
    const favoriteColor = "red";

    // Make a transaction to write to the blockchain
    let tx: string | null = null;
    try {
      tx = await program.methods
        // Call the set_favorites instruction handler
        .setFavorites(favoriteNumber, favoriteColor)
        .accounts({
          user: user.publicKey,
          // Note that both `favorites` and `system_program` are added
          // automatically.
        })
        // Sign the transaction
        .signers([user])
        // Send the transaction to the cluster or RPC
        .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );

    // And make sure it matches!
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(favoriteColor);
    expect(dataFromPda.number.toNumber()).toEqual(favoriteNumber.toNumber());
  });

  it("Updates our favorites on the blockchain", async () => {
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;
  
    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
      anchor.getProvider().connection,
      user.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );
  
    // Here's what we want to write to the blockchain
    const favoriteNumber = new anchor.BN(42);
    const favoriteColor = "blue";
  
    // Make a transaction to write to the blockchain
    let tx: string | null = null;
    try {
      tx = await program.methods
        // Call the set_favorites instruction handler
        .setFavorites(favoriteNumber, favoriteColor)
        .accounts({ user: user.publicKey })
        // Sign the transaction
        .signers([user])
        // Send the transaction to the cluster or RPC
        .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }
  
    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );
  
    // And make sure it matches!
    let dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.number.toNumber()).toBe(42);
    expect(dataFromPda.color).toBe("blue");
  
    // Now update only the number
    const updatedNumber = new anchor.BN(99);
    try {
      tx = await program.methods
        .updateFavorites(updatedNumber, null)
        .accounts({ user: user.publicKey, })
        .signers([user])
        .rpc();
    } catch (thrownObject) {
      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }
  
    console.log(`Tx signature: ${tx}`);
  
    dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.number.toNumber()).toBe(99); // updated
    expect(dataFromPda.color).toBe("blue");         // unchanged
  
    // Now update only the color
    const updatedColor = "green";
    try {
      tx = await program.methods
        .updateFavorites(null, updatedColor)
        .accounts({ user: user.publicKey, })
        .signers([user])
        .rpc();
    } catch (thrownObject) {
      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }
  
    console.log(`Tx signature: ${tx}`);
  
    dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.number.toNumber()).toBe(99);   // unchanged
    expect(dataFromPda.color).toBe("green");          // updated
  });
});
