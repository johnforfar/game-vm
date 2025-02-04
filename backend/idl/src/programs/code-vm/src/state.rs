use anchor_lang::prelude::*;
use crate::{types::*, MAX_NAME_LEN};

pub type RelayHistory = CircularBuffer<32, 32>;
pub type RelayTree = MerkleTree<64>;
pub type CompressedState = MerkleTree<24>;

#[account]
#[repr(C, align(8))]
#[derive(Copy, Debug, PartialEq)]
pub struct CodeVmAccount {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub slot: u64,
    pub poh: Hash,
    pub omnibus: TokenPool,
    pub lock_duration: u8,  // in days
    pub bump: u8,

    _padding: [u8; 5],
}


#[account]
#[repr(C, align(8))]
#[derive(Copy, Debug, PartialEq)]
pub struct MemoryAccount {
    pub vm: Pubkey,
    pub name: [u8; MAX_NAME_LEN],
    pub bump: u8,

    pub packed_info: [u8; 8],
    //pub _data: PhantomData<[u8]>,
}

#[account]
#[repr(C, align(8))]
#[derive(Copy, Debug, PartialEq)]
pub struct RelayAccount {
    pub vm: Pubkey,
    pub name: [u8; MAX_NAME_LEN],

    pub treasury: TokenPool,
    pub bump: u8,
    pub num_levels: u8,
    pub num_history: u8,

    _padding: [u8; 4],

    pub recent_roots: RelayHistory,
    pub history: RelayTree,
}

#[account]
#[repr(C, align(8))]
#[derive(Copy, Debug, PartialEq)]
pub struct StorageAccount {
    pub vm: Pubkey,
    pub name: [u8; MAX_NAME_LEN],
    pub bump: u8,
    pub depth: u8,

    _padding: [u8; 6],

    pub compressed_state: CompressedState,
}

#[account]
#[repr(C, align(8))]
#[derive(Copy, Debug, PartialEq)]
pub struct UnlockStateAccount {
    pub vm: Pubkey,
    pub owner: Pubkey,
    pub address: Pubkey,
    pub unlock_at: i64,
    pub bump: u8,
    pub state: u8,

    _padding: [u8; 6],
}

#[account]
#[repr(C, align(8))]
#[derive(Copy, Debug, PartialEq)]
pub struct WithdrawReceiptAccount {
    pub unlock_pda: Pubkey,
    pub nonce: Hash,
    pub amount: u64,
    pub bump: u8,

    _padding: [u8; 7],
}

pub mod game_pool {
    use super::*;
    use anchor_lang::prelude::Pubkey;

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
}