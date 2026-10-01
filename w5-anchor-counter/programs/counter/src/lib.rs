//! Counter на Anchor: счётчик с владельцем (authority).

pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;

// Program ID: публичный ключ из target/deploy/counter-keypair.json (anchor keys list).
declare_id!("673AWrposEH54uTFKn9fEi7L2z2jGEKG72F2CuDqG85E");

#[program]
pub mod counter {
    use super::*;

    /// Создаёт аккаунт Counter: count = 0, authority = подписант.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handle_initialize(ctx)
    }

    /// count += 1 (только authority).
    pub fn increment(ctx: Context<Update>) -> Result<()> {
        instructions::update::handle_increment(ctx)
    }

    /// count -= 1, ниже нуля нельзя (только authority).
    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        instructions::update::handle_decrement(ctx)
    }

    /// count = 0 (только authority).
    pub fn reset(ctx: Context<Update>) -> Result<()> {
        instructions::update::handle_reset(ctx)
    }
}
