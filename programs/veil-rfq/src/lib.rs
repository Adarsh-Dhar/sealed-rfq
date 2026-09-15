use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod veil_rfq {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, asset_mint: Pubkey, settlement_mint: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.asset_mint = asset_mint;
        config.settlement_mint = settlement_mint;
        config.bump = ctx.bumps.config;
        Ok(())
    }

    pub fn set_eligibility(ctx: Context<SetEligibility>, expires_at: i64, role: ParticipantRole) -> Result<()> {
        require!(expires_at > Clock::get()?.unix_timestamp, VeilError::InvalidExpiry);
        let eligibility = &mut ctx.accounts.eligibility;
        eligibility.config = ctx.accounts.config.key();
        eligibility.wallet = ctx.accounts.wallet.key();
        eligibility.expires_at = expires_at;
        eligibility.role = role;
        eligibility.bump = ctx.bumps.eligibility;
        Ok(())
    }

    pub fn create_rfq(ctx: Context<CreateRfq>, nonce: u64, request_commitment: [u8; 32], expires_at: i64) -> Result<()> {
        assert_active(&ctx.accounts.requester_eligibility)?;
        require!(expires_at > Clock::get()?.unix_timestamp, VeilError::InvalidExpiry);
        let rfq = &mut ctx.accounts.rfq;
        rfq.config = ctx.accounts.config.key();
        rfq.requester = ctx.accounts.requester.key();
        rfq.nonce = nonce;
        rfq.request_commitment = request_commitment;
        rfq.expires_at = expires_at;
        rfq.status = RfqStatus::Open;
        rfq.bump = ctx.bumps.rfq;
        Ok(())
    }

    pub fn submit_quote(ctx: Context<SubmitQuote>, quote_commitment: [u8; 32]) -> Result<()> {
        assert_active(&ctx.accounts.maker_eligibility)?;
        let rfq = &ctx.accounts.rfq;
        require!(rfq.status == RfqStatus::Open, VeilError::RfqNotOpen);
        require!(Clock::get()?.unix_timestamp <= rfq.expires_at, VeilError::RfqExpired);
        let quote = &mut ctx.accounts.quote;
        quote.rfq = rfq.key();
        quote.maker = ctx.accounts.maker.key();
        quote.quote_commitment = quote_commitment;
        quote.submitted_at = Clock::get()?.unix_timestamp;
        quote.bump = ctx.bumps.quote;
        Ok(())
    }

    pub fn select_quote(ctx: Context<SelectQuote>) -> Result<()> {
        assert_active(&ctx.accounts.requester_eligibility)?;
        assert_active(&ctx.accounts.maker_eligibility)?;
        let rfq = &mut ctx.accounts.rfq;
        require!(rfq.status == RfqStatus::Open, VeilError::RfqNotOpen);
        require!(Clock::get()?.unix_timestamp <= rfq.expires_at, VeilError::RfqExpired);
        require!(ctx.accounts.quote.rfq == rfq.key(), VeilError::QuoteForDifferentRfq);
        rfq.status = RfqStatus::Selected;
        rfq.selected_maker = Some(ctx.accounts.quote.maker);
        rfq.selected_quote_commitment = Some(ctx.accounts.quote.quote_commitment);
        Ok(())
    }

    pub fn expire_rfq(ctx: Context<ExpireRfq>) -> Result<()> {
        let rfq = &mut ctx.accounts.rfq;
        require!(rfq.status == RfqStatus::Open, VeilError::RfqNotOpen);
        require!(Clock::get()?.unix_timestamp > rfq.expires_at, VeilError::RfqStillLive);
        rfq.status = RfqStatus::Expired;
        Ok(())
    }
}

fn assert_active(eligibility: &Eligibility) -> Result<()> {
    require!(eligibility.expires_at >= Clock::get()?.unix_timestamp, VeilError::EligibilityExpired);
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + Config::LEN, seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetEligibility<'info> {
    #[account(mut, has_one = authority)]
    pub config: Account<'info, Config>,
    pub authority: Signer<'info>,
    /// CHECK: the issuer chooses this wallet; it is never deserialized or invoked.
    pub wallet: UncheckedAccount<'info>,
    #[account(init, payer = authority, space = 8 + Eligibility::LEN, seeds = [b"eligibility", config.key().as_ref(), wallet.key().as_ref()], bump)]
    pub eligibility: Account<'info, Eligibility>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct CreateRfq<'info> {
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub requester: Signer<'info>,
    #[account(seeds = [b"eligibility", config.key().as_ref(), requester.key().as_ref()], bump = requester_eligibility.bump, constraint = requester_eligibility.wallet == requester.key() @ VeilError::WrongEligibilityWallet)]
    pub requester_eligibility: Account<'info, Eligibility>,
    #[account(init, payer = requester, space = 8 + Rfq::LEN, seeds = [b"rfq", config.key().as_ref(), requester.key().as_ref(), &nonce.to_le_bytes()], bump)]
    pub rfq: Account<'info, Rfq>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitQuote<'info> {
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(seeds = [b"eligibility", config.key().as_ref(), maker.key().as_ref()], bump = maker_eligibility.bump, constraint = maker_eligibility.wallet == maker.key() @ VeilError::WrongEligibilityWallet)]
    pub maker_eligibility: Account<'info, Eligibility>,
    #[account(mut, constraint = rfq.config == config.key() @ VeilError::WrongConfig)]
    pub rfq: Account<'info, Rfq>,
    #[account(init, payer = maker, space = 8 + Quote::LEN, seeds = [b"quote", rfq.key().as_ref(), maker.key().as_ref()], bump)]
    pub quote: Account<'info, Quote>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SelectQuote<'info> {
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub requester: Signer<'info>,
    #[account(seeds = [b"eligibility", config.key().as_ref(), requester.key().as_ref()], bump = requester_eligibility.bump, constraint = requester_eligibility.wallet == requester.key() @ VeilError::WrongEligibilityWallet)]
    pub requester_eligibility: Account<'info, Eligibility>,
    #[account(mut, has_one = requester, constraint = rfq.config == config.key() @ VeilError::WrongConfig)]
    pub rfq: Account<'info, Rfq>,
    #[account(constraint = quote.rfq == rfq.key() @ VeilError::QuoteForDifferentRfq)]
    pub quote: Account<'info, Quote>,
    /// CHECK: bound to quote.maker and eligibility PDA; no data is read directly.
    pub maker: UncheckedAccount<'info>,
    #[account(seeds = [b"eligibility", config.key().as_ref(), maker.key().as_ref()], bump = maker_eligibility.bump, constraint = maker_eligibility.wallet == quote.maker @ VeilError::WrongEligibilityWallet)]
    pub maker_eligibility: Account<'info, Eligibility>,
}

#[derive(Accounts)]
pub struct ExpireRfq<'info> {
    #[account(mut)]
    pub rfq: Account<'info, Rfq>,
}

#[account]
pub struct Config { pub authority: Pubkey, pub asset_mint: Pubkey, pub settlement_mint: Pubkey, pub bump: u8 }
impl Config { pub const LEN: usize = 32 + 32 + 32 + 1; }

#[account]
pub struct Eligibility { pub config: Pubkey, pub wallet: Pubkey, pub expires_at: i64, pub role: ParticipantRole, pub bump: u8 }
impl Eligibility { pub const LEN: usize = 32 + 32 + 8 + 1 + 1; }

#[account]
pub struct Rfq {
    pub config: Pubkey,
    pub requester: Pubkey,
    pub nonce: u64,
    pub request_commitment: [u8; 32],
    pub expires_at: i64,
    pub status: RfqStatus,
    pub selected_maker: Option<Pubkey>,
    pub selected_quote_commitment: Option<[u8; 32]>,
    pub bump: u8,
}
impl Rfq { pub const LEN: usize = 32 + 32 + 8 + 32 + 8 + 1 + (1 + 32) + (1 + 32) + 1; }

#[account]
pub struct Quote { pub rfq: Pubkey, pub maker: Pubkey, pub quote_commitment: [u8; 32], pub submitted_at: i64, pub bump: u8 }
impl Quote { pub const LEN: usize = 32 + 32 + 32 + 8 + 1; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ParticipantRole { Investor, Maker, Issuer }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RfqStatus { Open, Selected, Expired }

#[error_code]
pub enum VeilError {
    #[msg("The eligibility record is expired.")] EligibilityExpired,
    #[msg("Expiry must be in the future.")] InvalidExpiry,
    #[msg("RFQ is not open.")] RfqNotOpen,
    #[msg("RFQ has expired.")] RfqExpired,
    #[msg("RFQ is still live.")] RfqStillLive,
    #[msg("Eligibility record does not belong to this wallet.")] WrongEligibilityWallet,
    #[msg("Account belongs to a different config.")] WrongConfig,
    #[msg("Quote belongs to a different RFQ.")] QuoteForDifferentRfq,
}
