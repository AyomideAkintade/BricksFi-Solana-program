import * as anchor from "@coral-xyz/anchor";
const { SystemProgram } = anchor.web3;
import { assert } from "chai";

describe('access_verification', () => {
  // Set up the local provider.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  // Load the program
  const program = anchor.workspace.AccessVerification;

  // Generate a new keypair for the user account
  const userAccount = anchor.web3.Keypair.generate();

  it('Initializes the user account', async () => {
    // Step 1: Initialize the UserAccount on-chain with no users initially
    await program.methods
      .initializeUserAccount()
      .accounts({
        userAccount: userAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([userAccount])
      .rpc();

    // Fetch the account data after initialization
    const account = await program.account.userAccount.fetch(userAccount.publicKey);
    assert.isArray(account.users);
    assert.equal(account.users.length, 0);  // Ensure it starts empty
  });

  it('Adds a user to the user account', async () => {
    // Step 2: Add a user to the UserAccount
    const newUser = anchor.web3.Keypair.generate(); // Create a new user keypair

    await program.methods
      .addUser(newUser.publicKey)
      .accounts({
        userAccount: userAccount.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    // Fetch the updated user account
    const account = await program.account.userAccount.fetch(userAccount.publicKey);

    // Check that the user's public key was added
    assert.equal(account.users.length, 1);
    assert.equal(account.users[0].toString(), newUser.publicKey.toString());
  });
});
