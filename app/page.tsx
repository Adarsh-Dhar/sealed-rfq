"use client";

import { useMemo, useState } from "react";
import { useClient } from "@solana/react";
import { useConnectedWallet } from "@solana/kit-plugin-wallet/react";
import { WalletConnect } from "./components/WalletConnect";

type Stage = "Draft" | "Quoted" | "Selected" | "Settled";
type View = "participant" | "issuer" | "observer";

const quotes = [
  { maker: "Northstar Markets", price: "98.62", id: "q-7f2a" },
  { maker: "Horizon Credit", price: "98.48", id: "q-24c9" }
];

export default function Home() {
  const [stage, setStage] = useState<Stage>("Draft");
  const [view, setView] = useState<View>("participant");
  const commitment = useMemo(() => "0x5d8a…a991", []);
  const selected = quotes[1];
  const client = useClient();
  const connectedWallet = useConnectedWallet();
  const next = () => setStage(current => current === "Draft" ? "Quoted" : current === "Quoted" ? "Selected" : "Settled");
  const button = !connectedWallet
    ? "Connect wallet to begin"
    : stage === "Draft" ? "Request sealed quotes"
    : stage === "Quoted" ? "Select best eligible quote"
    : stage === "Selected" ? "Settle delivery versus payment"
    : "Settlement final";
  const handleClick = () => {
    if (!connectedWallet) return; // WalletConnect in the nav handles the actual connect
    next();
  };

  // Smoke test only — proves wallet signing + broadcast works before any
  // veil_rfq-specific instructions are wired up. Remove once create_rfq /
  // submit_quote calls replace it.
  const testSend = async () => {
    if (!connectedWallet) return;
    const address = connectedWallet.accounts[0].address;
    const signature = await client.sendTransaction([
      client.system.instructions.transfer({
        source: address,
        destination: address,
        amount: 1n,
      }),
    ]);
    console.log("Sent:", signature);
  };

  return (
    <main>
      <nav><span className="mark">V</span><strong>Veil RFQ</strong><span className="tag">DEVNET</span><span className="cluster">Solana · Token-2022</span><WalletConnect /></nav>
      <section className="hero">
        <p className="eyebrow">PRIVATE CAPITAL MARKETS, ON-CHAIN</p>
        <h1>Trade permissioned RWAs<br />without broadcasting intent.</h1>
        <p className="lede">A sealed RFQ workflow for approved counterparties. Quotes are committed, eligibility is enforced, and settlement is designed for atomic delivery versus payment.</p>
      </section>
      <section className="views" aria-label="Privacy views">
        {(["participant", "issuer", "observer"] as View[]).map(item => <button key={item} onClick={() => setView(item)} className={view === item ? "active" : ""}>{item === "participant" ? "Trading party" : item === "issuer" ? "Issuer / auditor" : "Public observer"}</button>)}
      </section>
      <section className="grid">
        <article className="card request">
          <div className="card-head"><p className="eyebrow">RFQ-042</p><span className={`status ${stage.toLowerCase()}`}>{stage}</span></div>
          <h2>Atlas Private Credit Note</h2>
          <p className="muted">Token-2022 · Permissioned · USD settlement</p>
          <dl>
            <div><dt>Side</dt><dd>{view === "observer" ? "Hidden" : "Buy"}</dd></div>
            <div><dt>Notional</dt><dd>{view === "observer" ? "Confidential" : "$250,000"}</dd></div>
            <div><dt>Eligibility</dt><dd>{view === "observer" ? "Verified" : "Accredited US professional investor"}</dd></div>
            <div><dt>RFQ commitment</dt><dd className="mono">{commitment}</dd></div>
          </dl>
          {stage !== "Settled" && <button className="primary" onClick={handleClick}>{button}</button>}
          {stage === "Settled" && <p className="success">✓ Valid policy proof and settlement commitment recorded.</p>}
          {connectedWallet && <button className="secondary" onClick={testSend}>Send test transaction (1 lamport)</button>}
        </article>
        <article className="card quotes">
          <div className="card-head"><p className="eyebrow">SEALED LIQUIDITY</p><span className="muted">2 approved makers</span></div>
          {quotes.map((quote, index) => {
            const revealed = view !== "observer" && (stage === "Quoted" || stage === "Selected" || stage === "Settled");
            const winner = stage === "Selected" || stage === "Settled" ? index === 1 : false;
            return <div className={`quote ${winner ? "winner" : ""}`} key={quote.id}>
              <div><strong>{view === "observer" ? "Approved liquidity provider" : quote.maker}</strong><small className="mono">commitment {quote.id}</small></div>
              <div className="price">{revealed ? `$${quote.price}` : "Sealed"}{winner && <small>selected</small>}</div>
            </div>;
          })}
          <p className="note">Losing quote terms remain sealed after selection. The first on-chain MVP stores commitments; confidential compute and Token-2022 proof generation are the next integration milestone.</p>
        </article>
      </section>
      <section className="rail">
        {[["1", "Eligibility proof", "Approved without exposing full KYC"], ["2", "Sealed quotes", "Makers commit executable pricing"], ["3", "Policy-valid selection", "Best eligible quote is selected"], ["4", "Atomic settlement", "Asset and payment settle together"]].map(([n, title, detail]) => <div key={n}><span>{n}</span><strong>{title}</strong><p>{detail}</p></div>)}
      </section>
      <footer>Prototype only · No wallet signing or mainnet settlement is enabled.</footer>
    </main>
  );
}
