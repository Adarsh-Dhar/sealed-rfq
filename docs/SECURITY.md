# Security review checklist

This is an unaudited prototype. Do not deploy it with value.

- All state-bearing accounts are Anchor accounts with discriminators and program-owner checks.
- RFQ and Quote addresses are deterministic PDAs; a maker can submit one quote per RFQ.
- RFQ creation, quoting, and selection require a live issuer-created eligibility PDA.
- Selection checks requester ownership, config equality, quote-to-RFQ equality, and expiry.
- The program stores cryptographic commitments only. It never treats a commitment as an encrypted quote or proof of correctness.
- A production settlement instruction must validate token-program IDs, mint addresses, source/destination accounts, decimal precision, authority, and identical-asset rejection before any CPI.
- A confidential-transfer integration must verify all Token-2022 ZK context accounts and mint auditor configuration; it must be separately audited.
