use anchor_lang::prelude::*;

pub mod data;
pub mod errors;
pub mod methods;

use methods::*;

declare_id!("3Wbb4musZigEyTpbz287zufocXyi4ugjbGmp7Dr6khs9");

#[program]
pub mod r15_shared_messageboard {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::initialize(ctx)
    }

    pub fn create_message(ctx: Context<CreateMessage>, content: String) -> Result<()> {
        create_message::create_message(ctx, content)
    }

    pub fn add_allowed_user(ctx: Context<AddAllowedUser>) -> Result<()> {
        add_allowed_users::add_allowed_user(ctx)
    }

    pub fn generate_random_number(ctx: Context<GenerateRandomNumber>) -> Result<()> {
        generate_random_number::generate_random_number(ctx)
    }

    pub fn transfer_solana(ctx: Context<TransferSolana>, amount: u64) -> Result<()> {
        transfer_solana::transfer_solana(ctx, amount)
    }
}
