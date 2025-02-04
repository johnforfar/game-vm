use anchor_lang::prelude::*;

mod consts;
mod types;
mod state;
mod args;
mod instructions;

use consts::*;
use types::*;
use args::*;
use instructions::*;
// Removed game pool re-export to avoid duplicate definitions.
// pub use instructions::game_pool::{JoinPool, SubmitScore, ResolvePool};

declare_id!("vmZ1WUq8SxjBWcaeTCvgJRZbS84R61uniFsQy5YMRTJ");

#[program]
pub mod code_vm {
    use super::*;
    pub fn init_vm(_ctx: Context<InitVm>, _data: InitVmArgs) -> Result<()> {
        Ok(())
    }
    pub fn init_memory(_ctx: Context<InitMemory>, _data: InitMemoryArgs) -> Result<()> {
        Ok(())
    }
    pub fn init_storage(_ctx: Context<InitStorage>, _data: InitStorageArgs) -> Result<()> {
        Ok(())
    }
    pub fn init_relay(_ctx: Context<InitRelay>, _data: InitRelayArgs) -> Result<()> {
        Ok(())
    }
    pub fn init_nonce(_ctx: Context<InitNonce>, _data: InitNonceArgs) -> Result<()> {
        Ok(())
    }
    pub fn init_timelock(_ctx: Context<InitTimelock>, _data: InitTimelockArgs) -> Result<()> {
        Ok(())
    }
    pub fn init_unlock(_ctx: Context<InitUnlock>, _data: InitUnlockArgs) -> Result<()> {
        Ok(())
    }
    pub fn exec(_ctx: Context<Exec>, _data: ExecArgs) -> Result<()> {
        Ok(())
    }
    pub fn compress(_ctx: Context<Compress>, _data: CompressArgs) -> Result<()> {
        Ok(())
    }
    pub fn decompress(_ctx: Context<Decompress>, _data: DecompressArgs) -> Result<()> {
        Ok(())
    }
    pub fn resize_memory(_ctx: Context<ResizeMemory>, _data: ResizeMemoryArgs) -> Result<()> {
        Ok(())
    }
    pub fn snapshot(_ctx: Context<Snapshot>, _data: SnapshotArgs) -> Result<()> {
        Ok(())
    }
    pub fn deposit(_ctx: Context<Deposit>, _data: DepositArgs) -> Result<()> {
        Ok(())
    }
    pub fn withdraw(_ctx: Context<Withdraw>, _data: WithdrawArgs) -> Result<()> {
        Ok(())
    }
    pub fn unlock(_ctx: Context<Unlock>, _data: UnlockArgs) -> Result<()> {
        Ok(())
    }
    // The game pool instructions (join_pool, submit_score, resolve_pool) have been removed
    // from this IDL build. They are now solely declared in the program crate.
}

#[account]
pub struct GamePoolAccount {
    // Fixed to 3 players maximum. Unused slots will be all zeros.
    pub players: [Pubkey; 3],
    // Corresponding scores for each player.
    pub scores: [u64; 3],
    // Number of players who have joined.
    pub num_players: u8,
    // The game fee that each player pays.
    pub fee: u64,
    // True once the pool has been resolved.
    pub resolved: bool,
    // The index (0,1,2) of the winner – if resolved.
    pub winner_index: Option<u8>,
}

impl GamePoolAccount {
    // Adjust size calculation as needed.
    pub const LEN: usize = 3 * 32   // players
                           + 3 * 8    // scores
                           + 1        // num_players
                           + 8        // fee
                           + 1        // resolved (as a bool)
                           + 1;       // winner_index (Option<u8> serialized as one byte)
}