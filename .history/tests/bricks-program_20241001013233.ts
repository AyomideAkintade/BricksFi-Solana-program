import anchor from '@coral-xyz/anchor';
const { SystemProgram } = anchor.web3;
import { assert } from "chai";

describe('access_verification', () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.AccessVerification;
  const userAccount = anchor.web3.Keypair.generate();

  it('Initializes the user account', async () => {
    await program.methods
      .initializeUserAccount()
      .accounts({
        userAccount: userAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([userAccount])
      .rpc();

    const account = await program.account.userAccount.fetch(userAccount.publicKey);
    assert.isArray(account.users);
    assert.equal(account.users.length, 0);  // Ensure it starts empty
  });

  it('Adds a user to the user account', async () => {
    const newUser = anchor.web3.Keypair.generate(); // Create a new user keypair

    await program.methods
      .addUser(newUser.publicKey)
      .accounts({
        userAccount: userAccount.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.userAccount.fetch(userAccount.publicKey);
    assert.equal(account.users.length, 1);
    assert.equal(account.users[0].toString(), newUser.publicKey.toString());
  });
});
