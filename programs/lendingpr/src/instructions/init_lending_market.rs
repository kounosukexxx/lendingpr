use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::token::Token;

use crate::{account_data::lending_market::*, utils::account_length::AccountLength};

#[derive(Accounts)]
pub struct InitLendingMarket<'info> {

  // pay するからmut 今からつくるmarketのowner. program_idとは違う
  #[account(mut)]
  pub owner: Signer<'info>,

  // initを使うとそらなはアカウント自体は作ってくれる。残りのメタデータを自分でこれから作る
  // Marks the account as mutable and is mutually exclusive with mut.
  // Makes the account rent exempt unless skipped with rent_exempt = skip.
  #[account(
    init,
    payer = owner,
    // 8バイト：なんかの織別しをつけるために必要(account discriminator)
    space = LendingMarket::LEN + 8,
  )]
  pub lending_market: Account<'info, LendingMarket>,

  // cpi叩くのに必要
  pub system_program: Program<'info, System>,

  pub token_program: Program<'info, Token>,

  // TODO: uncheckってなぜに
  pub oracle: UncheckedAccount<'info>
}

pub fn process_init_lending_market(
  ctx: Context<InitLendingMarket>,
  quote_currency: [u8; 32],
) -> ProgramResult {
  let lending_market = &ctx.accounts.lending_market.key();
  ctx.accounts.lending_market.init(InitLendingMarketParams {
    bump_seed: Pubkey::find_program_address(&[lending_market.as_ref()], ctx.program_id).1,
    owner: ctx.accounts.owner.key(),
    quote_currency,
    token_program_id: ctx.accounts.token_program.key(),
    oracle_program_id: ctx.accounts.oracle.key(),
  });

  Ok(())
}

// program_idはレンディングマーケットをつくるというサービス自体のことで、レンディングマーケット自体を作るという権限をもつaccount