use anchor_lang::prelude::*;
#[cfg(not(feature = "no-entrypoint"))]
use {default_env::default_env, solana_security_txt::security_txt};

pub mod constants;
pub mod contexts;
pub mod error;
pub mod events;
pub mod states;


use contexts::*;

declare_id!("LFBbSSCPVnJddba5SbWkJAfPxhdjpqajMTq1LM2Q5A9");

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "LF Coin",
    project_url: "https://lfcoin.io",
    contacts: "Email: info@lfcoin.io, X: https://x.com/lfcoindao, Telegram: https://t.me/lfcoindao, Instagram: https://instagram.com/LFCoin.io",
    policy: "https://lfcoin.io/lfcoin_whitepaper.pdf",

    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/lfcoin-io/LFCoin-Smart-Contract",
}

#[program]
pub mod token_biu {
    use super::*;

    pub fn initialize_sale(
        ctx: Context<InitializeSale>,
        token_price_usd: f64,
        mint_decimals: u64,
        purchase_limit: u64,
    ) -> Result<()> {
        ctx.accounts.initialize(token_price_usd, mint_decimals, purchase_limit)
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()> {
        ctx.accounts.buy(sol_amount)
    }

    pub fn change_reciepent_account(ctx: Context<AdminControl>, new_receipent: Pubkey) -> Result<()> {
        ctx.accounts.change_recipient(new_receipent)
    }

    pub fn change_config_authority(ctx: Context<AdminControl>, new_authority: Pubkey) -> Result<()> {
        ctx.accounts.change_authority(new_authority)
    }

    pub fn set_purchase_limit(ctx: Context<AdminControl>, new_limit: u64) -> Result<()> {
        ctx.accounts.set_limit(new_limit)
    }

    pub fn pause_sale(ctx: Context<AdminControl>) -> Result<()> {
        ctx.accounts.pause()
    }

    pub fn resume_sale(ctx: Context<AdminControl>) -> Result<()> {
        ctx.accounts.resume()
    }

    pub fn set_monthly_limits(ctx: Context<SetMonthlyLimits>, limits: [u64; 14], timestamps: [i64; 14]) -> Result<()> {
        ctx.accounts.set_limits(limits, timestamps)
    }

    pub fn enable_vesting(ctx: Context<VestingControl>) -> Result<()> {
        ctx.accounts.enable_vesting()
    }

    pub fn disable_vesting(ctx: Context<VestingControl>) -> Result<()> {
        ctx.accounts.disable_vesting()
    }

    pub fn withdraw_tokens(ctx: Context<WithdrawTokens>, token_amount: u64) -> Result<()> {
        ctx.accounts.withdraw_remaining_tokens(token_amount)
    }
}
