import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PiggyBank } from "../target/types/piggy_bank";
import { assert } from "chai";

describe("piggy-bank", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PiggyBank as Program<PiggyBank>;
  const user = provider.wallet.publicKey;

  const [piggyBankPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("piggy-bank"), user.toBuffer()],
    program.programId
  );

  it("Initializes the piggy bank", async () => {
    const accountInfo = await provider.connection.getAccountInfo(piggyBankPda);
    
    if (accountInfo === null) {
      await program.methods
        .initialize()
        .accounts({
          piggyBank: piggyBankPda,
          user: user,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
      console.log("Piggy bank initialized at:", piggyBankPda.toBase58());
    } else {
      console.log("Piggy bank already exists, skipping initialization.");
    }

    const account = await program.account.piggyBank.fetch(piggyBankPda);
    assert.equal(account.owner.toBase58(), user.toBase58());
  });

  it("Deposits SOL", async () => {
    const depositAmount = new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL);
    const balanceBefore = await provider.connection.getBalance(piggyBankPda);

    await program.methods
      .deposit(depositAmount)
      .accounts({
        piggyBank: piggyBankPda,
        user: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    const balanceAfter = await provider.connection.getBalance(piggyBankPda);
    assert.isAbove(balanceAfter, balanceBefore);
    console.log("Deposited. New PDA balance:", balanceAfter / anchor.web3.LAMPORTS_PER_SOL, "SOL");
  });

  it("Withdraws SOL", async () => {
    const withdrawAmount = new anchor.BN(0.05 * anchor.web3.LAMPORTS_PER_SOL);
    const userBefore = await provider.connection.getBalance(user);

    await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        piggyBank: piggyBankPda,
        owner: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    const userAfter = await provider.connection.getBalance(user);
    assert.isAbove(userAfter, userBefore - 10000000); 
    console.log("Withdrew. User balance:", userAfter / anchor.web3.LAMPORTS_PER_SOL, "SOL");
  });
});