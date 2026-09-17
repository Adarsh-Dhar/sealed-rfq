use anchor_lang::prelude::*;

declare_id!("FMCRZK6bpr1GYx5W2TxCPVjXwW4N1FbDmzGE3yh4Atyd");

#[program]
pub mod veil_rfq {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
