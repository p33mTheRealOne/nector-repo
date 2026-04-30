use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::{Order, EscrowAccount, ErrorCode, OrderState};

#[derive(Accounts)]
#[instruction(order_index: u64)]
pub struct SellerFundEscrow<'info> {

    #[account(
        mut,
        seeds = [b"order", seller.key().as_ref(), &order_index.to_le_bytes()],
        bump
    )]
    pub order: Account<'info, Order>,

    #[account(
        mut,
        seeds = [b"escrow", order.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        address = pubkey!("GCcZkwkhGhzqBt6Eoc2nJCZFvgYdFAnh1hWuuARi774Z")// You can change to your own fee wallet
    )]
    /// CHECK: platform fee wallet
    pub fee_wallet: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn seller_fund_escrow_handler(
    ctx: Context<SellerFundEscrow>,
    _order_index: u64,
) -> Result<()> {

    let order = &mut ctx.accounts.order;

    // clone account info ก่อน borrow mut
    let escrow_account_info = ctx.accounts.escrow.to_account_info();

    let escrow = &mut ctx.accounts.escrow;

    let clock = Clock::get()?;

    // Only Seller
    require!(
        ctx.accounts.seller.key() == order.seller_wallet,
        ErrorCode::InvalidSeller
    );

    // Only state = BuyerFunded
    require!(
        order.state == OrderState::BuyerFunded as u8,
        ErrorCode::InvalidState
    );

    // read price from order
    let price = order.price_lamports;

    // Calculate Bond depends on Draw Dispu te Mode
    let bond = match order.mode {
        0 => price * 20 / 100,   // BTR
        1 => price * 120 / 100,  // STR
        _ => return err!(ErrorCode::InvalidMode),
    };

    // Calculate 1%
    let fee = price * 1 / 100;

    // ---------------- TRANSFER BOND -> ESCROW ----------------
    let bond_transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.seller.to_account_info(),
            to: escrow_account_info.clone(),
        },
    );

    transfer(bond_transfer_ctx, bond)?;

    // ---------------- TRANSFER FEE -> FEE WALLET ----------------
    let fee_transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.seller.to_account_info(),
            to: ctx.accounts.fee_wallet.to_account_info(),
        },
    );

    transfer(fee_transfer_ctx, fee)?;

    escrow.amount_locked += bond;

    // save economics to order
    order.bond_lamports = bond;
    order.fee_lamports += fee;
    order.total_lamports += bond;
    order.seller_funded_at = clock.unix_timestamp;
    // Change state to SellerFunded
    order.state = OrderState::SellerFunded as u8;

    Ok(())
}