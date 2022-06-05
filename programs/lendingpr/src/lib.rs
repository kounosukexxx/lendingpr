// 発信するやつら
pub mod account_data;
pub mod constants;
pub mod instructions;
pub mod utils;

use crate::instructions::init_lending_market::*;
use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[program]
// pub mod lendingpr {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }

//     pub fn init_lending_market(
//         ctx: Coutext<InitLendingMarket>,
//         quote_currency: [u8; 32],
//     ) -> ProgramResult {
//         process_init_lending_market(ctx, quote_currency)
//     }
// }

#[derive(Accounts)]
pub struct Initialize {}
