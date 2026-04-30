use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;

declare_id!("9buv2Wao6udvrauoot3JuGGJ8cB3ULfX2r1BzVNw1h64");

#[program]
pub mod nector {
    use super::*;

    pub fn init_seller(ctx: Context<InitSeller>) -> Result<()> {
        let seller_account = &mut ctx.accounts.seller_account;
        seller_account.owner = ctx.accounts.seller.key();
        seller_account.order_count = 0;
        Ok(())
    }

    pub fn buyer_fund_escrow(
        ctx: Context<BuyerFundEscrow>,
        order_index: u64,
    ) -> Result<()> {
        fund_escrow_handler(ctx, order_index)
    }

    pub fn buyer_cancel(
        ctx: Context<BuyerCancel>,
        order_index: u64,
    ) -> Result<()> {
        buyer_cancel_handler(ctx, order_index)
    }

    pub fn seller_fund_escrow(
        ctx: Context<SellerFundEscrow>,
        order_index: u64,
    ) -> Result<()> {
        seller_fund_escrow_handler(ctx, order_index)
    }

    pub fn shipping_timeout(
        ctx: Context<ShippingTimeout>,
        order_index: u64,
    ) -> Result<()> {
        shipping_timeout_handler(ctx, order_index)
    }

    pub fn mark_shipped(
        ctx: Context<MarkShipped>,
        order_index: u64,
    ) -> Result<()> {
        mark_shipped_handler(ctx, order_index)
    }

    pub fn confirm_delivery(
        ctx: Context<ConfirmDelivery>,
        order_index: u64,
    ) -> Result<()> {
        confirm_delivery_handler(ctx, order_index)
    }

    pub fn confirm_timeout(
        ctx: Context<ConfirmTimeout>,
        order_index: u64,
    ) -> Result<()> {
        confirm_timeout_handler(ctx, order_index)
    }

    pub fn open_dispute(
        ctx: Context<OpenDispute>,
        order_index: u64,
    ) -> Result<()> {
        open_dispute_handler(ctx, order_index)
    }

    pub fn refund_buyer(
        ctx: Context<RefundBuyer>,
        order_index: u64,
    ) -> Result<()> {
        refund_buyer_handler(ctx, order_index)
    }

    pub fn respond_dispute(
        ctx: Context<RespondDispute>,
        order_index: u64,
    ) -> Result<()> {
        respond_dispute_handler(ctx, order_index)
    }

    pub fn buyer_win(
        ctx: Context<BuyerWin>,
        order_index: u64,
    ) -> Result<()> {
        buyer_win_handler(ctx, order_index)
    }

    pub fn draw(
        ctx: Context<Draw>,
        order_index: u64,
    ) -> Result<()> {
        draw_handler(ctx, order_index)
    }

    pub fn seller_cancel(
        ctx: Context<SellerCancel>,
        order_index: u64,
    ) -> Result<()> {
        seller_cancel_handler(ctx, order_index)
    }

    pub fn refund_during_discuss(
        ctx: Context<RefundDuringDiscuss>,
        order_index: u64,
    ) -> Result<()> {
        refund_during_discuss_handler(ctx, order_index)
    }

    pub fn pay_seller_during_discuss(
        ctx: Context<PaySellerDuringDiscuss>,
        order_index: u64,
    ) -> Result<()> {
        pay_seller_during_discuss_handler(ctx, order_index)
    }

    pub fn create_order(
        ctx: Context<CreateOrder>,
        order_index: u64,
        mode: u8,
        product_type: u8,
        order_name: String,
        buyer_wallet: Pubkey,
        price_lamports: u64,
        shipping_hours: u32,
    ) -> Result<()> {
        // ---------------- validate enums ----------------
        match mode {
            0 | 1 => {}
            _ => return err!(ErrorCode::InvalidMode),
        };

        if product_type == ProductType::Physical as u8 {
            require!(
                shipping_hours > 0 && shipping_hours <= 720,
                ErrorCode::InvalidShippingTime
            );
        }

        if product_type == ProductType::Digital as u8 {
            require!(
                shipping_hours > 0 && shipping_hours <= 48,
                ErrorCode::InvalidShippingTime
            );
        }

        if product_type == ProductType::Digital as u8 {
            require!(
                shipping_hours > 0 && shipping_hours <= 48,
                ErrorCode::InvalidShippingTime
            );
        }

        if product_type == ProductType::Digital as u8 {
            require!(mode == Mode::BTR as u8, ErrorCode::InvalidModeForDigital);
        }
        
        match product_type {
            0 | 1 => {}
            _ => return err!(ErrorCode::InvalidProductType),
        };

        let seller_account = &mut ctx.accounts.seller_account;

        // Change Index
        require!(
            order_index == seller_account.order_count,
            ErrorCode::InvalidOrderIndex
        );

        // ---------------- create order ----------------
        let order = &mut ctx.accounts.order;
        order.order_index = order_index;
        order.mode = mode;
        order.product_type = product_type;
        order.state = 0; // CREATED
        order.order_name = order_name;
        order.buyer_wallet = buyer_wallet;
        order.seller_wallet = ctx.accounts.seller.key();
        order.price_lamports = price_lamports;
        order.shipping_hours = shipping_hours;

        // ---------------- increment counter ----------------
        seller_account.order_count += 1;

        Ok(())
    }
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Mode {
    BTR = 0,
    STR = 1,
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProductType {
    Physical = 0,
    Digital = 1,
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum OrderState {
    Created = 0,
    BuyerFunded = 1,
    SellerFunded = 2,
    Cancelled = 3,
    ShippingTimedOut = 4,
    MarkShipped = 5,
    Completed = 6,
    OpenDispute = 7,
    Refunded = 8,
    BuyerWonDispute = 9,
    SellerResponded = 10,
    Draw = 11,
}

#[account]
pub struct SellerAccount {
    pub owner: Pubkey,
    pub order_count: u64,
}

impl SellerAccount {
    pub const SIZE: usize = 32 + 8;
}

#[account]
pub struct Order {
    pub mode: u8,
    pub product_type: u8,
    pub order_name: String,
    pub buyer_wallet: Pubkey,
    pub seller_wallet: Pubkey,
    pub price_lamports: u64,
    pub shipping_hours: u32,
    pub state: u8,
    pub order_index: u64,
    pub bond_lamports: u64,
    pub fee_lamports: u64,
    pub total_lamports: u64,
    pub seller_funded_at: i64,
    pub mark_shipped_at: i64,
    pub open_dispute_at: i64,
    pub seller_respond_at: i64,
    }

impl Order {
    pub const MAX_NAME: usize = 100;

    pub const SIZE: usize =
        1 +
        1 +
        4 + Self::MAX_NAME +
        32 +
        32 +
        8 +
        4 +
        1 +
        8 +
        8 +
        8 +
        8 +
        8 +
        8 +
        8;
}

#[account]
pub struct EscrowAccount {
    pub order: Pubkey,
    pub buyer: Pubkey,
    pub amount_locked: u64,
}

impl EscrowAccount {
    pub const SIZE: usize = 32 + 32 + 8;
}

#[derive(Accounts)]
#[instruction(order_index: u64)]
pub struct CreateOrder<'info> {

    #[account(
        mut,
        seeds = [b"seller", seller.key().as_ref()],
        bump
    )]
    pub seller_account: Account<'info, SellerAccount>,

    #[account(
        init,
        payer = seller,
        space = 8 + Order::SIZE,
        seeds = [
            b"order",
            seller.key().as_ref(),
            &order_index.to_le_bytes()
        ],
        bump
    )]
    pub order: Account<'info, Order>,

    #[account(mut)]
    pub seller: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitSeller<'info> {
    #[account(
        init,
        payer = seller,
        seeds = [b"seller", seller.key().as_ref()],
        bump,
        space = 8 + SellerAccount::SIZE
    )]
    pub seller_account: Account<'info, SellerAccount>,

    #[account(mut)]
    pub seller: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid mode")]
    InvalidMode,
    #[msg("Invalid product type")]
    InvalidProductType,
    #[msg("Only Seller can trigger")]
    InvalidSeller,
    #[msg("Invalid order index")]
    InvalidOrderIndex,
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Digital product must use BTR mode")]
    InvalidModeForDigital,
    #[msg("Invalid shipping time")]
    InvalidShippingTime,
    #[msg("Digital product cannot have shipping time")]
    DigitalNoShipping,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Only Buyer can trigger")]
    InvalidBuyer,
    #[msg("Order already funded")]
    AlreadyFunded,
    #[msg("Invalid State")]
    InvalidState,
    #[msg("Shipping deadline not reached")]
    ShippingNotExpired,
    #[msg("Insufficient Escrow")]
    InsufficientEscrow,
    #[msg("Confirm deadline not reached")]
    ConfirmNotExpired,
    #[msg("Only buyer can open dispute")]
    OnlyBuyerCanOpenDispute,
    #[msg("Dispute deadline not reached")]
    DisputeDeadlineNotReached,
    #[msg("Discussion deadline not reached")]
    DiscussionNotReached,
}