use anchor_lang::prelude::*;
use crate::{Order, EscrowAccount, ErrorCode, OrderState};

#[derive(Accounts)]
#[instruction(order_index: u64)]
pub struct ConfirmTimeout<'info> {

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
}

pub fn confirm_timeout_handler(
    ctx: Context<ConfirmTimeout>,
    _order_index: u64,
) -> Result<()> {

    let order = &mut ctx.accounts.order;
    let escrow = &mut ctx.accounts.escrow;

    let clock = Clock::get()?;

    // ---------------- state check ----------------

    require!(
        order.state == OrderState::MarkShipped as u8,
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

    // ---------------- deadline check ----------------

    let confirm_deadline =
        order.mark_shipped_at + 24 * 3600;

    require!(
        clock.unix_timestamp >= confirm_deadline,
        ErrorCode::ConfirmNotExpired
    );

    // ---------------- economics ----------------

    let price = order.price_lamports;

    let buyer_bond = price * 20 / 100;

    let seller_bond = order.bond_lamports;

    let seller_payment = price;
    let buyer_refund = buyer_bond;
    let seller_refund = seller_bond;

    let total =
        seller_payment +
        buyer_refund +
        seller_refund;

    require!(
        escrow.amount_locked >= total,
        ErrorCode::InsufficientEscrow
    );

    // ---------------- transfers ----------------

    // seller gets price
    **escrow.to_account_info().try_borrow_mut_lamports()? -= seller_payment;
    **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += seller_payment;

    // buyer gets bond
    **escrow.to_account_info().try_borrow_mut_lamports()? -= buyer_refund;
    **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? += buyer_refund;

    // seller gets bond back
    **escrow.to_account_info().try_borrow_mut_lamports()? -= seller_refund;
    **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += seller_refund;

    // ---------------- state ----------------

    order.state = OrderState::Completed as u8;

    Ok(())
}