use anchor_lang::prelude::*;

declare_id!("DdhwnKpzGuTsfcwUC6CDyU9YNNaf96vA8n18W5e7EdwU");

#[program]
pub mod test_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.user.id = 1;
        ctx.accounts.user.owner = ctx.accounts.signer.key();
        Ok(())
    }

    pub fn update(ctx: Context<Update>, new_id: u64, verify: u64) -> Result<()> {
        require!(verify % 5 == 0, ErrorCode::NotDivisibleBy4); 
        // oh no, we made a typo! should be % 4 not 5, ex: will allow 15 through when 15 is not divisible by 4
        ctx.accounts.user.id = new_id;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, space = 8 + 32 + 8)]
    pub user: Account<'info, User>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, constraint = user.owner == signer.key())]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub user: Account<'info, User>,
}

#[account]
pub struct User {
    pub owner: Pubkey,
    pub id: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("not divisble by 4")]
    NotDivisibleBy4,
}