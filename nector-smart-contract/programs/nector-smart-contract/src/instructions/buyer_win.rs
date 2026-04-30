use anchor_lang::prelude::*;
use crate::{Order, EscrowAccount, ErrorCode, OrderState};

#[derive(Accounts)]
#[instruction(order_index: u64)]
pub struct BuyerWin<'info> {
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
    /// CHECK: must match order.buyer_wallet
    pub buyer: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: must match order.seller_wallet
    pub seller: UncheckedAccount<'info>,

    #[account(
        mut,
        address = pubkey!("GCcZkwkhGhzqBt6Eoc2nJCZFvgYdFAnh1hWuuARi774Z")// You can change to your own fee wallet
    )]
    pub fee_wallet: SystemAccount<'info>,
}

pub fn buyer_win_handler(
    ctx: Context<BuyerWin>,
    _order_index: u64,
) -> Result<()> {
    let order = &mut ctx.accounts.order;
    let escrow = &mut ctx.accounts.escrow;
    let clock = Clock::get()?;

    require!(
        order.state == OrderState::OpenDispute as u8,
        ErrorCode::InvalidState
    );

    require!(
        ctx.accounts.buyer.key() == order.buyer_wallet,
        ErrorCode::InvalidBuyer
    );

    require!(
        ctx.accounts.seller.key() == order.seller_wallet,
        ErrorCode::InvalidSeller
    );

    let respond_deadline = order.open_dispute_at + 24 * 3600;

    require!(
        clock.unix_timestamp >= respond_deadline,
        ErrorCode::DisputeDeadlineNotReached
    );

    let price = order.price_lamports;
    let buyer_deposit = price + (price * 20 / 100);
    let seller_deposit = order.bond_lamports;
    let penalty = price * 20 / 100;

    let buyer_reward = penalty / 2;
    let fee_amount = penalty / 2;

    let seller_refund = seller_deposit
        .checked_sub(penalty)
        .unwrap_or(0);

    let total = buyer_deposit + buyer_reward + fee_amount + seller_refund;

    require!(
        escrow.amount_locked >= total,
        ErrorCode::InsufficientEscrow
    );

    **escrow.to_account_info().try_borrow_mut_lamports()? -= buyer_deposit;
    **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? += buyer_deposit;

    **escrow.to_account_info().try_borrow_mut_lamports()? -= buyer_reward;
    **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? += buyer_reward;

    **escrow.to_account_info().try_borrow_mut_lamports()? -= fee_amount;
    **ctx.accounts.fee_wallet.to_account_info().try_borrow_mut_lamports()? += fee_amount;

    if seller_refund > 0 {
        **escrow.to_account_info().try_borrow_mut_lamports()? -= seller_refund;
        **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += seller_refund;
    }

    escrow.amount_locked = 0;
    order.state = OrderState::BuyerWonDispute as u8;

    Ok(())
}