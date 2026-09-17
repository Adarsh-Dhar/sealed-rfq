use anchor_lang::prelude::*;

declare_id!("8zScwwxadJ35AFaB3cHi3ZZwQLg6KzjGGUWFRFx5izGb");

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
