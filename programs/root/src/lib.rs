use anchor_lang::prelude::*;

declare_id!("J3YrmcktoZowvWnCpKYaKT9ehasfQ8KxbNhoe6ZGPGfA");

#[program]
mod root {
    use super::*;

    pub fn initialize(ctx: Context<Create>) -> Result<()> {
        let slots_acc = &mut ctx.accounts.slots_acc;

        if slots_acc.initialized {
            return Err(MyError::AlreadyInitialized.into());
        }

        slots_acc.authority = ctx.accounts.user.key.clone();
        slots_acc.initialized = true;
        slots_acc.slots = Vec::new();
        Ok(())
    }

    pub fn addroots<'info>(
        ctx: Context<'_, '_, '_, 'info, AddRoots<'info>>,
        slot: u64,
        mt_root: [u8; 32],
        ws_root: [u8; 32],
    ) -> Result<()> {
        // todo: add authority, only special account can call this ix
        let slots_acc = &mut ctx.accounts.slots_acc;
        slots_acc.slots.push(slot);

        let slot_roots_acc = &mut ctx.accounts.slot_roots_acc;
        slot_roots_acc.merkle_tree_root = mt_root;
        slot_roots_acc.world_state_root = ws_root;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 10240)]
    pub slots_acc: Account<'info, SlotsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(slot: u64)]
pub struct AddRoots<'info> {
    #[account(mut)]
    pub slots_acc: Account<'info, SlotsAccount>,
    pub system_program: Program<'info, System>,
    #[account(
        init_if_needed, 
        payer = authority,
        space = 8 + SlotRootsAccout::INIT_SPACE,
        seeds = [b"roots", slot.to_le_bytes().as_ref()],
        bump)
    ]
    pub slot_roots_acc: Account<'info, SlotRootsAccout>,
    #[account(mut)]
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
pub struct SlotsAccount {
    pub authority: Pubkey,
    pub initialized: bool,
    pub slots: Vec<u64>,
}

#[account]
#[derive(InitSpace)]
pub struct SlotRootsAccout{
    pub slot: u64,
    pub merkle_tree_root: [u8; 32],
    pub world_state_root: [u8; 32],
}
