import { AnchorProvider, Program, web3 } from "@coral-xyz/anchor";
import "./App.css";
import idl from "../../target/idl/counter_anchor.json";
import { useAnchorWallet, useWallet } from "@solana/wallet-adapter-react";
import { useState } from "react";

const programID = new web3.PublicKey("5Wnpeo6hA75WGacPT9THrf1GFYsUky5JnEfh2b91tgkc");

function App() {
  const wallet = useAnchorWallet()
  const { publicKey } = useWallet()
  const [count, setCount] = useState<number | null>(null)
  const [account, setAccount] = useState<web3.PublicKey | null>(null)

  const getProvider = () => {
    const connection = new web3.Connection("https://api.devnet.solana.com")
    return new AnchorProvider(connection, wallet!, { preflightCommitment: "processed" })
  }

  const initCounter = async () => {
    const provider = getProvider()
    const program = new Program(idl, programID, provider)

    const counterAccount = web3.Keypair.generate()

    await program.methods.initialize().accounts({
      newAccount: counterAccount.publicKey,
      signer: provider.wallet.publicKey,
      systemProgram: web3.SystemProgram.programId,
    }).signers([counterAccount]).rpc()

    setAccount(counterAccount.publicKey)

    const acc = await program.account.accountInstructions.fetch(counterAccount.publicKey)
    setCount(acc.count.toNumber());
  }

  const increment = async () => {
    if (!account) return;

    const provider = getProvider()
    const program = new Program(idl, programID, provider)

    await program.methods.increment().accounts({
      newAccount: account,
      owner: provider.wallet.publicKey
    }).rpc()

    const acc = await program.account.accountInstructions.fetch(account)
    setCount(acc.count.toNumber())
  }

  return (
    <div className=" p-20" >

      {!publicKey ? (
        <button onClick={() => "connect wallet using phantom wallet at top"}>Connect Wallet</button>
      ) : <div>
        <button onClick={initCounter}>Initialize Counter</button>
        <button onClick={increment} disabled={!account}> increment</button>
        <h3>Count: {count ?? "not initialized"}</h3>
      </div>}
    </div>
  )
}

export default App;
