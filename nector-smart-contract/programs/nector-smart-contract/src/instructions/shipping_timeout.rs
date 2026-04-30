use anchor_lang::prelude::*;
use crate::{Order, EscrowAccount, ErrorCode, OrderState};

#[derive(Accounts)]
#[instruction(order_index: u64)]
pub struct ShippingTimeout<'info> {

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

    /// CHECK
    #[account(mut)]
    pub buyer: UncheckedAccount<'info>,

    /// CHECK
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,

    #[account(
        mut,
        address = pubkey!("GCcZkwkhGhzqBt6Eoc2nJCZFvgYdFAnh1hWuuARi774Z")// You can change to your own fee wallet
    )]
    /// CHECK
    pub fee_wallet: SystemAccount<'info>,
}

pub fn shipping_timeout_handler(
    ctx: Context<ShippingTimeout>,
    _order_index: u64,
) -> Result<()> {

    let order = &mut ctx.accounts.order;
    let escrow = &mut ctx.accounts.escrow;

    let clock = Clock::get()?;

    require!(
        order.state == OrderState::SellerFunded as u8,
        ErrorCode::InvalidState
    );

    // verify buyer / seller
    require!(
        ctx.accounts.buyer.key() == order.buyer_wallet,
        ErrorCode::InvalidBuyer
    );

    require!(
        ctx.accounts.seller.key() == order.seller_wallet,
        ErrorCode::InvalidSeller
    );

    // ---------------- check shipping deadline ----------------

    let deadline =
        order.seller_funded_at +
        (order.shipping_hours as i64) * 3600;

    require!(
        clock.unix_timestamp >= deadline,
        ErrorCode::ShippingNotExpired
    );

    // ---------------- economics ----------------

    let price = order.price_lamports;
    let penalty = price * 20 / 100;

    let buyer_bonus = penalty / 2;
    let fee_amount = penalty / 2;

    let buyer_deposit = price + (price * 20 / 100);

    let seller_bond = order.bond_lamports;

    let seller_refund = seller_bond
        .checked_sub(penalty)
        .unwrap_or(0);

    let total_needed =
        buyer_deposit +
        buyer_bonus +
        fee_amount +
        seller_refund;

    require!(
        escrow.amount_locked >= total_needed,
        ErrorCode::InsufficientEscrow
    );

    // ---------------- transfer lamports ----------------

    // buyer refund
    **escrow.to_account_info().try_borrow_mut_lamports()? -= buyer_deposit;
    **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? += buyer_deposit;

    // buyer penalty reward
    **escrow.to_account_info().try_borrow_mut_lamports()? -= buyer_bonus;
    **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? += buyer_bonus;

    // fee
    **escrow.to_account_info().try_borrow_mut_lamports()? -= fee_amount;
    **ctx.accounts.fee_wallet.to_account_info().try_borrow_mut_lamports()? += fee_amount;

    // seller refund (STR case)
    if seller_refund > 0 {
        **escrow.to_account_info().try_borrow_mut_lamports()? -= seller_refund;
        **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += seller_refund;
    }

    order.state = OrderState::ShippingTimedOut as u8;

    Ok(())
}