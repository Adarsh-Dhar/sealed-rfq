use anchor_lang::prelude::*;

declare_id!("6gRGKyvy4qJjSCBcxh6uaaSeih1ZxVCDt926Y3Fp8BVC");

#[program]
pub mod veil_rfq_match {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initialize veil-rfq-match program");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
