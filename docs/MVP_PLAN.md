# Veil RFQ MVP plan

## One-sentence product

Veil RFQ lets an approved investor request sealed block quotes for a permissioned Token-2022 asset, select an eligible maker, and settle delivery versus payment with an issuer audit trail.

## Hackathon demo scope

### Must work

1. An issuer allowlists investor and maker wallets.
2. An investor creates an RFQ commitment; the public state does not carry the human-readable notional or quote terms.
3. Two allowlisted makers submit quote commitments before expiry.
4. The investor selects one maker using an off-chain decrypted quote payload plus the on-chain commitment.
5. The program records selection and creates a settlement intent.
6. The UI renders trading-party, issuer/auditor, and public-observer views.

### Devnet proof target

- Base DvP: both legs transfer in one transaction using Token-2022-compatible token interfaces.
- Confidential extension: demonstrate a separate Token-2022 confidential transfer for an approved test mint, including the issuer auditor key. Do not represent this as atomic with the base DvP until the proof/context-account integration is complete.

## Explicit non-goals

- Mainnet, custody, KYC collection, broker-dealer operations, legal compliance claims, credit underwriting, anonymous networking, matching across third-party order books, and production security.

## Architecture

```text
Next.js demo UI
  ├─ wallet + sealed payload client (next milestone)
  ├─ RFQ / quote commitment client
  └─ role-based visibility renderer

Anchor program
  ├─ Config PDA: issuer authority + permitted mints
  ├─ Eligibility PDA: wallet approval and expiry
  ├─ RFQ PDA: requester, commitment, expiry, status
  ├─ Quote PDA: maker, commitment, submitted slot
  └─ SettlementIntent PDA: selected quote commitment + audit hash

Token-2022
  ├─ transfer-hook / allowlist policy (issuer integration)
  └─ confidential transfer amount/balance privacy (devnet extension milestone)
```

## Four-week delivery plan

### Week 1 — executable core

- Create and test the Anchor RFQ state machine.
- Add deterministic PDA derivations and authorization constraints.
- Ship the role-aware demo UI and README.

### Week 2 — devnet transaction proof

- Deploy to devnet only after review.
- Create Token-2022 test mint and allowlisted accounts.
- Submit/create/select an RFQ through a connected devnet wallet.

### Week 3 — confidentiality integration

- Generate confidential-transfer proofs client-side.
- Set up a mint auditor key and recipient confidential accounts.
- Show a devnet amount-confidential transfer; retain normal DvP fallback if proof integration is not stable.

### Week 4 — evidence and submission

- Add integration tests and a security review checklist.
- Record the 90-second observer / issuer / trader demo.
- Publish an open-source repository, architecture diagram, and disclosure of the confidentiality boundary.

## Acceptance criteria

- No expired RFQ can accept a quote or be selected.
- Only approved participants can create RFQs, quote, or select.
- A quote can be submitted once per maker/RFQ pair.
- RFQ state transitions are monotonic: Open → Selected → Settled or Expired.
- Public UI never displays raw notional, quote price, or credential in observer mode.

## Risk register

| Risk | Mitigation |
| --- | --- |
| Token-2022 confidential transfer complexity | Treat as a devnet proof milestone; do not block the sealed-RFQ state machine. |
| Privacy overclaim | UI and README label the exact public and private fields. |
| RFQ quote copying / reveal | Store commitments on-chain and encrypt quote payloads to the requester. |
| Eligibility replay | Bind credentials and eligibility PDAs to wallet plus expiry. |
| On-chain account substitution | Use Anchor typed accounts, PDA seeds, signer checks, and `has_one` constraints. |
