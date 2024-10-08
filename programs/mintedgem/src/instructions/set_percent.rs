use anchor_lang::prelude::*;

use crate::{constants::MASTER, errors::CustomErrors, events::SetPercent, state::Master};

#[derive(Accounts)]
pub struct SetPercentCtx<'info> {
    #[account(
        mut,
        seeds = [MASTER],
        bump
    )]
    master: Account<'info, Master>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn process(ctx: Context<SetPercentCtx>, percent: u16) -> Result<()> {
    require!(percent <= 10000, CustomErrors::InvalidPercent);

    require_keys_eq!(
        ctx.accounts.master.owner,
        ctx.accounts.signer.key(),
        CustomErrors::NotOwner
    );

    ctx.accounts.master.percent = percent;

    emit!(SetPercent { percent });

    Ok(())
}
