use anchor_lang::prelude::*;

declare_id!("5vAQqq7ZFfoL4orYsUKnUbSaJqmYeh88PQCuj451531F");

#[program]
pub mod veil_rfq {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
