#!/usr/bin/env bash
# Run this from the root of your veil-rfq-match project
# (the directory arcium init veil-rfq-match created).
set -euo pipefail

if [ ! -d "encrypted-ixs" ] || [ ! -d "programs/veil-rfq-match" ]; then
  echo "Error: run this from the veil-rfq-match project root" \
       "(expected ./encrypted-ixs and ./programs/veil-rfq-match to exist)."
  exit 1
fi

echo "Backing up existing files to *.bak ..."
cp encrypted-ixs/src/lib.rs encrypted-ixs/src/lib.rs.bak 2>/dev/null || true
cp programs/veil-rfq-match/src/lib.rs programs/veil-rfq-match/src/lib.rs.bak 2>/dev/null || true

echo "Writing encrypted-ixs/src/lib.rs ..."
cat > encrypted-ixs/src/lib.rs << 'EOF'
use arcis::*;

#[encrypted]
mod circuits {
    use arcis::*;

    pub struct SealedQuote {
        maker_index: u8,
        price_bps: u32,
    }

    pub struct BestQuote {
        winner_index: u8,
        winner_price_bps: u32,
    }

    #[instruction]
    pub fn select_best_quote(
        quote_a: Enc<Shared, SealedQuote>,
        quote_b: Enc<Shared, SealedQuote>,
    ) -> Enc<Shared, BestQuote> {
        let a = quote_a.to_arcis();
        let b = quote_b.to_arcis();

        let a_wins = a.price_bps < b.price_bps; // lowest price wins, matches lib/rfq.ts
        let winner_index = a_wins.select(a.maker_index, b.maker_index);
        let winner_price_bps = a_wins.select(a.price_bps, b.price_bps);

        quote_a.owner.from_arcis(BestQuote {
            winner_index,
            winner_price_bps,
        })
    }
}
EOF

echo "Writing programs/veil-rfq-match/src/lib.rs ..."
cat > programs/veil-rfq-match/src/lib.rs << 'EOF'
use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;

const COMP_DEF_OFFSET_SELECT_BEST: u32 = comp_def_offset("select_best_quote");

declare_id!("YOUR_NEW_MXE_PROGRAM_ID_HERE");

#[arcium_program]
pub mod veil_rfq_match {
    use super::*;

    // Run once, after deployment, before the first real call
    pub fn init_select_best_comp_def(ctx: Context<InitSelectBestCompDef>) -> Result<()> {
        init_computation_def(ctx.accounts, None)?;
        Ok(())
    }

    // Called per-RFQ once both sealed quotes are in
    pub fn select_best_quote(
        ctx: Context<SelectBestQuote>,
        computation_offset: u64,
        quote_a_ciphertext: [u8; 32],
        quote_b_ciphertext: [u8; 32],
        pub_key: [u8; 32],
        nonce: u128,
    ) -> Result<()> {
        let args = ArgBuilder::new()
            .x25519_pubkey(pub_key)
            .plaintext_u128(nonce)
            .encrypted_u8(quote_a_ciphertext)
            .encrypted_u8(quote_b_ciphertext)
            .build();

        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![SelectBestQuoteCallback::callback_ix(
                computation_offset,
                &ctx.accounts.mxe_account,
                &[],
            )?],
            1,
            0,
            0,
        )?;
        Ok(())
    }

    // Invoked automatically once the MPC cluster finishes
    #[arcium_callback(encrypted_ix = "select_best_quote")]
    pub fn select_best_quote_callback(
        ctx: Context<SelectBestQuoteCallback>,
        output: SignedComputationOutputs<BestQuoteOutput>,
    ) -> Result<()> {
        let result = match output.verify_output(
            &ctx.accounts.cluster_account,
            &ctx.accounts.computation_account,
        ) {
            Ok(o) => o,
            Err(e) => {
                msg!("Error: {}", e);
                return Err(ErrorCode::AbortedComputation.into());
            }
        };

        emit!(BestQuoteSelectedEvent {
            ciphertext: result.ciphertexts[0],
            nonce: result.nonce.to_le_bytes(),
        });
        Ok(())
    }
}
EOF

echo "Done. Originals saved as lib.rs.bak next to each file."
echo "NOTE: account structs (InitSelectBestCompDef, SelectBestQuote, SelectBestQuoteCallback," \
     "BestQuoteOutput, ErrorCode, BestQuoteSelectedEvent) are NOT included here -- adapt those" \
     "from what arcium init already generated for you in this file's .bak backup, per the guide."
