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
