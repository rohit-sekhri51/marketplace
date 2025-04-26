pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use crate::error::MarketplaceError;

declare_id!("5VMRFa1m1pVRBB1yutT6Z2Z7Vfvhh7cGgBmisgzc9HX6");

/// # NFT Marketplace Program
///
/// This program allows users to create NFT marketplaces where NFTs can be listed and sold.
/// The program supports:
/// - Creating configurable marketplaces with custom fee structures
/// - Listing NFTs for sale with collection verification
/// - Purchasing NFTs with automatic fee distributions
/// - (Future) Reward token distribution for marketplace participants
#[program]
pub mod marketplace_rs {

    use super::*;

    /// Creates a new marketplace with the specified configuration
    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        // Handle validation
        require!(fee <= 10000, MarketplaceError::InvalidFee); // Max fee is 100% (10000 basis points)

        // Call the implementation function
        ctx.accounts.init(name, fee, &ctx.bumps)
    }
}
