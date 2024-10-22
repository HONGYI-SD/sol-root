use anchor_lang::prelude::*;

declare_id!("FJvoRMKLXxcFCNvAR1UGoVjiewpqEY7bRiHtfyHiBAWN");

#[program]
mod root {
    use super::*;

    pub fn initialize(ctx: Context<Create>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        if counter.initialized {
            return Err(MyError::AlreadyInitialized.into());
        }

        counter.authority = ctx.accounts.user.key.clone();
        counter.initialized = true;
        counter.merkle_data = Vec::new();
        Ok(())
    }

    pub fn insert(
        ctx: Context<Increment>,
        slot_height: u64,
        root1: [u8; 32],
        root2: [u8; 32],
    ) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(
            counter.authority == *ctx.accounts.authority.key,
            MyError::IncrementError
        );

        let entry = MerkleEntry {
            slot_height,
            root1,
            root2,
        };

        counter.merkle_data.push(entry); // 使用 Vec
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 10240)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[error_code]
pub enum MyError {
    #[msg("Only the deployer can call this function")]
    IncrementError,
    #[msg("The counter has already been initialized")]
    AlreadyInitialized,
    #[msg("Unauthorized to initialize")]
    Unauthorized,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub initialized: bool,
    pub merkle_data: Vec<MerkleEntry>, // 使用 Vec
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MerkleEntry {
    pub slot_height: u64,
    pub root1: [u8; 32],
    pub root2: [u8; 32],
}
