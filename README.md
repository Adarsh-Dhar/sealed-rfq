# Veil RFQ

Private request-for-quote workflow for permissioned Token-2022 real-world assets on Solana.

## MVP truth boundary

This repository implements the hackathon-safe core: permissioned RFQ lifecycle, sealed quote commitments, selection rules, an issuer audit view, and a devnet-ready atomic DvP interface for standard Token-2022 transfers. It does **not** claim that the initial program performs Token-2022 confidential-transfer proofs. Those transfers require client-generated ZK proof/context accounts and must be integrated and demonstrated on devnet before the product can claim confidential settlement.

See [docs/MVP_PLAN.md](docs/MVP_PLAN.md) for scope, architecture, milestones, and the proof plan.

## Run the demo

```bash
npm install
npm run dev
```

Open `http://localhost:3000`. The first demo is deliberately simulation-first: it makes the privacy boundary and lifecycle legible before a wallet signs anything.

## Verify

```bash
npm run lint
npm test
npm run build
```

## Security note

Never use a mainnet wallet or real tokenized assets with this prototype. The program is unaudited and no deployment is included.
# sealed-rfq
