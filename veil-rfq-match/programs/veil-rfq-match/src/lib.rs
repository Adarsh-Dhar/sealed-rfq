use anchor_lang::prelude::*;

declare_id!("hVEbidVqPG2bpP7uuevMoBBPwCxwQwH6vVeiSm3rBoG");

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
