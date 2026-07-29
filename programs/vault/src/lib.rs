use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.owner = ctx.accounts.owner.key();
        state.state_bump = ctx.bumps.state;
        state.vault_bump = ctx.bumps.vault;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
        );
        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let seeds = &[
            b"vault",
            ctx.accounts.owner.key.as_ref(),
            &[ctx.accounts.state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        let seeds = &[
            b"vault",
            ctx.accounts.owner.key.as_ref(),
            &[ctx.accounts.state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        transfer(cpi_ctx, ctx.accounts.vault.lamports())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"state", owner.key().as_ref()],
        bump
    )]
    pub state: Account<'info, VaultState>,

    /// CHECK: vault PDA holds SOL; system account
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        seeds = [b"state", owner.key().as_ref()],
        bump = state.state_bump
    )]
    pub state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        seeds = [b"state", owner.key().as_ref()],
        bump = state.state_bump
    )]
    pub state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", owner.key().as_ref()],
        bump = state.state_bump,
        close = owner
    )]
    pub state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub owner: Pubkey,
    pub state_bump: u8,
    pub vault_bump: u8,
}
