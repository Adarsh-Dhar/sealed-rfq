"use client";

import {
  useWallets,
  useConnect,
  useConnectedWallet,
  useDisconnect,
} from "@solana/kit-plugin-wallet/react";

export function WalletConnect() {
  const wallets = useWallets();          // Wallet Standard auto-discovery
  const connect = useConnect();
  const disconnect = useDisconnect();
  const connectedWallet = useConnectedWallet();

  if (connectedWallet) {
    const address = connectedWallet.accounts[0]?.address;
    return (
      <div className="wallet-status">
        <span className="mono">
          {address?.slice(0, 4)}…{address?.slice(-4)}
        </span>
        <button onClick={() => disconnect()}>Disconnect</button>
      </div>
    );
  }

  if (wallets.length === 0) {
    return (
      <span className="muted">
        No wallet detected — install Phantom or Solflare
      </span>
    );
  }

  return (
    <div className="wallet-list">
      {wallets.map((wallet) => (
        <button key={wallet.name} onClick={() => connect(wallet)}>
          Connect {wallet.name}
        </button>
      ))}
    </div>
  );
}
