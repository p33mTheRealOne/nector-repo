use anchor_lang::prelude::*;
use crate::{Order, EscrowAccount, OrderState, ErrorCode};

#[derive(Accounts)]
#[instruction(order_index: u64)]
pub struct SellerCancel<'info> {

    #[account(
        mut,
        seeds = [b"order", order.seller_wallet.as_ref(), &order_index.to_le_bytes()],
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
    /// CHECK
    pub buyer: UncheckedAccount<'info>,

    #[account(mut)]
    pub seller: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn seller_cancel_handler(
    ctx: Context<SellerCancel>,
    _order_index: u64,
) -> Result<()> {

    let order = &mut ctx.accounts.order;
    let escrow = &mut ctx.accounts.escrow;

    require!(
        order.seller_wallet == ctx.accounts.seller.key(),
        ErrorCode::Unauthorized
    );

    require!(
        order.state == OrderState::SellerFunded as u8,
        ErrorCode::InvalidState
    );

    require!(
        order.buyer_wallet == ctx.accounts.buyer.key(),
        ErrorCode::InvalidBuyer
    );

    let price = order.price_lamports;

    // buyer deposit = 120%
    let buyer_deposit = price + (price * 20 / 100);

    // seller deposit
    let seller_deposit = if order.mode == 0 {
        // BTR
        price * 20 / 100
    } else {
        // STR
        price + (price * 20 / 100)
    };

    // refund buyer
    **escrow.to_account_info().try_borrow_mut_lamports()? -= buyer_deposit;
    **ctx.accounts.buyer.try_borrow_mut_lamports()? += buyer_deposit;

    // refund seller
    **escrow.to_account_info().try_borrow_mut_lamports()? -= seller_deposit;
    **ctx.accounts.seller.try_borrow_mut_lamports()? += seller_deposit;

    order.state = OrderState::Cancelled as u8;

    Ok(())
}