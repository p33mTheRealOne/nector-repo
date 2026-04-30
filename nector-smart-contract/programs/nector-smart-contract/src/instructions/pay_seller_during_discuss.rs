use anchor_lang::prelude::*;
use crate::{Order, EscrowAccount, OrderState, ErrorCode};

#[derive(Accounts)]
#[instruction(order_index: u64)]
pub struct PaySellerDuringDiscuss<'info> {

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
    pub buyer: Signer<'info>,

    #[account(mut)]
    /// CHECK
    pub seller: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn pay_seller_during_discuss_handler(
    ctx: Context<PaySellerDuringDiscuss>,
    _order_index: u64,
) -> Result<()> {

    let order = &mut ctx.accounts.order;
    let escrow = &mut ctx.accounts.escrow;

    require!(
        order.buyer_wallet == ctx.accounts.buyer.key(),
        ErrorCode::Unauthorized
    );

    require!(
        order.seller_wallet == ctx.accounts.seller.key(),
        ErrorCode::InvalidSeller
    );

    require!(
        order.state == OrderState::SellerResponded as u8,
        ErrorCode::InvalidState
    );

    let price = order.price_lamports;

    let buyer_deposit = price + (price * 20 / 100);

    let seller_deposit = if order.mode == 0 {
        price * 20 / 100
    } else {
        price + (price * 20 / 100)
    };

    let buyer_bond = buyer_deposit - price;

    let seller_receive = price + seller_deposit;

    let buyer_receive = buyer_bond;

    **escrow.to_account_info().try_borrow_mut_lamports()? -= seller_receive;
    **ctx.accounts.seller.try_borrow_mut_lamports()? += seller_receive;

    **escrow.to_account_info().try_borrow_mut_lamports()? -= buyer_receive;
    **ctx.accounts.buyer.try_borrow_mut_lamports()? += buyer_receive;

    order.state = OrderState::Completed as u8;

    Ok(())
}