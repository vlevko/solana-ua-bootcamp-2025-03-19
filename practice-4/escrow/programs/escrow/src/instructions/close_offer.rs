use anchor_lang::prelude::*;

use anchor_spl::token_interface::{
    close_account,
    transfer_checked,
    CloseAccount,
    Mint,
    TokenAccount,
    TokenInterface,
    TransferChecked
};

use crate::Offer;

#[derive(Accounts)]
pub struct CloseOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = token_mint_a,
        seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn return_tokens_to_maker(ctx: &Context<CloseOffer>) -> Result<()> {
    let offer = &ctx.accounts.offer;
    let seeds = &[
        b"offer",
        ctx.accounts.maker.key.as_ref(),
        &offer.id.to_le_bytes(),
        &[offer.bump],
    ];
    let signer = &[&seeds[..]];

    // Transfer tokens back to maker
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.maker_token_account_a.to_account_info(),
            mint: ctx.accounts.token_mint_a.to_account_info(),
            authority: ctx.accounts.offer.to_account_info(),
        },
        signer,
    );

    transfer_checked(
        transfer_ctx,
        ctx.accounts.vault.amount,
        ctx.accounts.token_mint_a.decimals,
    )
}

pub fn close_offer_and_vault(ctx: Context<CloseOffer>) -> Result<()> {
    let offer = &ctx.accounts.offer;
    let seeds = &[
        b"offer",
        ctx.accounts.maker.key.as_ref(),
        &offer.id.to_le_bytes(),
        &[offer.bump],
    ];
    let signer = &[&seeds[..]];

    // Close the Vault account
    let close_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.vault.to_account_info(),
            destination: ctx.accounts.maker.to_account_info(),
            authority: ctx.accounts.offer.to_account_info(),
        },
        signer,
    );

    close_account(close_ctx)?;

    // Close the Offer account
    ctx.accounts.offer.close(ctx.accounts.maker.to_account_info())?;

    // Rent price is refunded to maker
    Ok(())
}
