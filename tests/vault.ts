import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { expect } from "chai";

describe("vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Vault as Program<Vault>;
  const owner = provider.wallet as anchor.Wallet;

  const [statePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), owner.publicKey.toBuffer()],
    program.programId
  );
  const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), owner.publicKey.toBuffer()],
    program.programId
  );

  const depositAmount = new anchor.BN(1_000_000_000); // 1 SOL
  const withdrawAmount = new anchor.BN(400_000_000); // 0.4 SOL

  it("initializes the vault", async () => {
    await program.methods
      .initialize()
      .accounts({
        owner: owner.publicKey,
        state: statePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const state = await program.account.vaultState.fetch(statePda);
    expect(state.owner.toBase58()).to.equal(owner.publicKey.toBase58());
  });

  it("deposits SOL into the vault", async () => {
    const before = await provider.connection.getBalance(vaultPda);

    await program.methods
      .deposit(depositAmount)
      .accounts({
        owner: owner.publicKey,
        state: statePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const after = await provider.connection.getBalance(vaultPda);
    expect(after - before).to.equal(depositAmount.toNumber());
  });

  it("withdraws SOL from the vault", async () => {
    const before = await provider.connection.getBalance(vaultPda);

    await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        owner: owner.publicKey,
        state: statePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const after = await provider.connection.getBalance(vaultPda);
    expect(before - after).to.equal(withdrawAmount.toNumber());
  });

  it("closes the vault", async () => {
    await program.methods
      .close()
      .accounts({
        owner: owner.publicKey,
        state: statePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const stateInfo = await provider.connection.getAccountInfo(statePda);
    expect(stateInfo).to.equal(null);

    const vaultBalance = await provider.connection.getBalance(vaultPda);
    expect(vaultBalance).to.equal(0);
  });
});
