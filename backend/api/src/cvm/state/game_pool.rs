// backend/api/src/cvm/state/game_pool.rs
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GamePoolAccount {
    // Fixed to 3 players maximum. Unused slots will be all zeros.
    pub players: [Pubkey; 3],
    // Corresponding scores for each player.
    pub scores: [u64; 3],
    // Number of players who have joined.
    pub num_players: u8,
    // The game fee that each player pays (e.g., 100 kin).
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