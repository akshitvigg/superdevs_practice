import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterAnchor } from "../target/types/counter_anchor";
import { assert } from "chai";

describe("counter-anchor", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.counterAnchor as Program<CounterAnchor>;

  let counterAccount: anchor.web3.Keypair;

  it("Initialize Counter!", async () => {
    counterAccount = anchor.web3.Keypair.generate()

    await program.methods.initialize().accounts({
      newAccount: counterAccount.publicKey,
      signer: provider.wallet.publicKey,
    }).signers([counterAccount]).rpc()

    const account = await program.account.accountInstructions.fetch(counterAccount.publicKey)
    assert.equal(account.count.toNumber(), 0)
  });

  it("increment counter", async () => {

    await program.methods.increment().accounts({
      newAccount: counterAccount.publicKey,
      //@ts-ignore
      owner: provider.wallet.publicKey
    }).rpc()

    const account = await program.account.accountInstructions.fetch(counterAccount.publicKey)
    assert.equal(account.count.toNumber(), 1);
  })

  it("decrement counter", async () => {

    await program.methods.decrement().accounts({
      newAccount: counterAccount.publicKey,
      //@ts-ignore
      owner: provider.wallet.publicKey
    }).rpc()

    const account = await program.account.accountInstructions.fetch(counterAccount.publicKey)
    assert.equal(account.count.toNumber(), 0)
  })

  it("reset counter", async () => {

    await program.methods.reset().accounts({
      newAccount: counterAccount.publicKey,
      //@ts-ignore
      owner: provider.wallet.publicKey
    }).rpc()

    const account = await program.account.accountInstructions.fetch(counterAccount.publicKey)
    assert.equal(account.count.toNumber(), 0)

  })
});
