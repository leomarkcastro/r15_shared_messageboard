import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { R15SharedMessageboard } from "../target/types/r15_shared_messageboard";

describe("r15_shared_messageboard", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .R15SharedMessageboard as Program<R15SharedMessageboard>;

  // load program keypair
  const appKeypair = anchor.web3.Keypair.generate();

  // load user account
  // const mainCreatorWallet = (program.provider as anchor.AnchorProvider).wallet;

  // create wallet from keypair
  const otherUser_1_Wallet = new anchor.Wallet(anchor.web3.Keypair.generate());
  const otherUser_2_Wallet = new anchor.Wallet(anchor.web3.Keypair.generate());

  // airdrop some solana to wallets
  before(async () => {
    // airdrop some solana
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        otherUser_1_Wallet.publicKey,
        10000000000
      ),
      "confirmed"
    );
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        otherUser_2_Wallet.publicKey,
        10000000000
      ),
      "confirmed"
    );
  });

  // afterEach(async () => {
  //   // get the state of the program
  //   const state = await program.account.messageBoard.fetch(
  //     appKeypair.publicKey
  //   );
  //   console.log("State", state);
  // });

  it.only("Is initialized!", async () => {
    // Add your test here.
    await program.methods
      .initialize()
      .accounts({
        messageBoard: appKeypair.publicKey,
        user: otherUser_1_Wallet.publicKey,
      })
      .signers([appKeypair, otherUser_1_Wallet.payer])
      .rpc();

    /**
     * Note:
     * So like at first, we initialize the app.
     * We create a new address that would handle the logic of the
     * script. [appKeypair], and he signs himself. Idk why. Maybe because he owns
     * himself, not the deployer.
     */
  });

  it("Author Can add new message", async () => {
    await program.methods
      .createMessage("Hello World from Author")
      .accounts({
        messageBoard: appKeypair.publicKey,
        user: otherUser_1_Wallet.publicKey,
      })
      .signers([otherUser_1_Wallet.payer])
      .rpc();

    /**
     * Note:
     * If the caller of the script is the original owner himself,
     * then the user and the signer user is assumed to be himself
     * (or was this some hardcode of anchor)
     */
  });

  it("Others can add new message if added as co-poster", async () => {
    // expect method to throw error since he's not allowed
    await expect(
      program.methods
        .createMessage("Hello World from other user")
        .accounts({
          messageBoard: appKeypair.publicKey,
          user: otherUser_2_Wallet.publicKey,
        })
        .signers([otherUser_2_Wallet.payer])
        .rpc()
    ).to.throw;

    // Co writers add address as allowed user
    await program.methods
      .addAllowedUser()
      .accounts({
        messageBoard: appKeypair.publicKey,
        user: otherUser_1_Wallet.publicKey,
        newUser: otherUser_2_Wallet.publicKey,
      })
      .signers([otherUser_1_Wallet.payer])
      .rpc();

    // now he can post
    await program.methods
      .createMessage("Hello World other signers!")
      .accounts({
        messageBoard: appKeypair.publicKey,
        user: otherUser_2_Wallet.publicKey,
      })
      .signers([otherUser_2_Wallet.payer])
      .rpc();
  });

  it("can generate random number", async () => {
    await program.methods
      .generateRandomNumber()
      .accounts({
        messageBoard: appKeypair.publicKey,
        user: otherUser_1_Wallet.publicKey,
      })
      .signers([otherUser_1_Wallet.payer])
      .rpc();
  });

  it.only("should be able to transfer solana", async () => {
    // check balance of user 2
    const balance = await program.provider.connection.getBalance(
      otherUser_2_Wallet.publicKey
    );
    console.log("Balance", balance);

    await program.methods
      .transferSolana(new anchor.BN(1000000000))
      .accounts({
        payer: otherUser_1_Wallet.publicKey,
        recipient: otherUser_2_Wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([otherUser_1_Wallet.payer])
      .rpc();

    // check balance of user 2
    const balanceAfter = await program.provider.connection.getBalance(
      otherUser_2_Wallet.publicKey
    );
    console.log("Balance After", balanceAfter);
  });
});
