
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod robo_swap_program {
    use super::*;

    pub fn delete(
        _ctx: Context<Delete>
    ) -> Result<()> {
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        for idx in 0..game::ROBOTS {
            let robot = &mut ctx.accounts.pda.robots[idx];
            *robot = game::Robot::new(ctx.accounts.user.key(), idx as u8)?;
        }
        ctx.accounts.pda.bump = *ctx.bumps.get("pda").unwrap_or_else(|| panic!("hundt"));
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, victim_idx: u8, robber_idx: u8) -> Result<()> {
        
        require!(robber_idx <= 25, game::RoboSwapError::IndexOutOfBounds);
        require!(victim_idx <= 25, game::RoboSwapError::IndexOutOfBounds);

        let r = &mut ctx.accounts.robber_pda.robots[robber_idx as usize];
        let v = &mut ctx.accounts.victim_pda.robots[victim_idx as usize];

        let helper = r.owner;
        r.owner = v.owner;
        v.owner = helper;

        let helper = r.owner_idx;
        r.owner_idx = v.owner_idx;
        v.owner_idx = helper;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Delete<'info> {
    pub system_program: Program<'info, System>,
    
    /// CHECK: This is not dangerous
    #[account(
        mut,
        seeds = [b"RoboSwap", receiver.key().as_ref()], bump,
        close = receiver,
    )]
    pub pda: Box<Account<'info, game::Game>>,

    #[account(
        mut,
    )]
    pub receiver: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, payer = user, space = 8 + game::GAME_SIZE, 
        seeds = [b"RoboSwap", user.key().as_ref()], bump,
    )]  
    pub pda: Box<Account<'info, game::Game>>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub robber: Signer<'info>,
    #[account(
        mut,
        seeds = [b"RoboSwap", robber.key().as_ref()], bump,
    )]  
    pub robber_pda: Box<Account<'info, game::Game>>,

    #[account(mut)]
    pub victim: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"RoboSwap", victim.key().as_ref()], bump,
    )]  
    pub victim_pda: Box<Account<'info, game::Game>>,
}

#[account]
pub struct NewAccount(u8);

mod game {
    use anchor_lang::prelude::*;
    use anchor_lang::{prelude::Pubkey, account};

    pub const ROBOTS: usize = 26;
    pub const ROBOT_SIZE: usize = 70;
    pub const GAME_SIZE: usize = ROBOT_SIZE * ROBOTS + 1;

    #[account]
    pub struct Game {
        pub robots: [Robot; ROBOTS],
        pub bump: u8,
    }
    impl Game {}

    #[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, PartialEq, Eq)]
    pub     struct Robot {
        pub wallet: Pubkey,
        pub owner: Pubkey,
        pub idx: u8,
        pub owner_idx: u8,
        pub swaps: u32,
    }
    impl Robot {
        pub fn new(wallet: Pubkey, idx: u8) -> Result<Self> {
            require!(idx <= 25, RoboSwapError::IndexOutOfBounds);
            Ok(Self {
                owner: wallet.clone(),
                wallet,
                idx,
                owner_idx: idx,
                swaps: 0,
            })
        }
    }

    #[error_code]
    pub enum RoboSwapError {
        IndexOutOfBounds,
        UserAndAccountNotEqual,
    }


}