// backend/program/src/instruction/game_pool.rs
use anchor_lang::prelude::*;
use crate::state::game_pool::GamePoolAccount;

#[derive(Accounts)]
#[instruction(fee: u64)]
pub struct JoinPool<'info> {
    // When the account does not exist, we initialize it with a PDA based on a pool ID.
    #[account(init_if_needed, payer = player, space = GamePoolAccount::LEN, seeds = [b"game_pool", pool_id.as_ref()], bump)]
    pub game_pool: Account<'info, GamePoolAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    /// CHECK: This is just a seed.
    pub pool_id: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn join_pool(ctx: Context<JoinPool>, fee: u64) -> Result<()> {
    let pool = &mut ctx.accounts.game_pool;
    // For simplicity, we assume fee is paid (your real implementation should verify token transfers)
    if pool.num_players >= 3 {
        return Err(ErrorCode::PoolFull.into());
    }
    pool.players[pool.num_players as usize] = *ctx.accounts.player.key;
    pool.scores[pool.num_players as usize] = 0;
    pool.num_players += 1;
    pool.fee = fee;
    Ok(())
}

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    #[account(mut)]
    pub game_pool: Account<'info, GamePoolAccount>,
    pub player: Signer<'info>,
}

pub fn submit_score(ctx: Context<SubmitScore>, score: u64) -> Result<()> {
    let pool = &mut ctx.accounts.game_pool;
    let mut found = false;
    for i in 0..(pool.num_players as usize) {
        if pool.players[i] == *ctx.accounts.player.key {
            pool.scores[i] = score;
            found = true;
            break;
        }
    }
    if !found {
        return Err(ErrorCode::PlayerNotInPool.into());
    }
    Ok(())
}

#[derive(Accounts)]
pub struct ResolvePool<'info> {
    #[account(mut)]
    pub game_pool: Account<'info, GamePoolAccount>,
}

pub fn resolve_pool(ctx: Context<ResolvePool>) -> Result<()> {
    let pool = &mut ctx.accounts.game_pool;
    if pool.num_players < 3 {
        return Err(ErrorCode::PoolNotFull.into());
    }
    if pool.resolved {
        return Err(ErrorCode::PoolAlreadyResolved.into());
    }
    let mut best_score = 0;
    let mut winner_index = 0;
    for i in 0..(pool.num_players as usize) {
        if pool.scores[i] > best_score {
            best_score = pool.scores[i];
            winner_index = i;
        }
    }
    pool.resolved = true;
    pool.winner_index = Some(winner_index as u8);
    // Here you would trigger the token transfer:
    // 90% of the total fees go to the winning player, and 10% to the developer.
    // (The actual transfer logic is omitted for clarity.)
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("The game pool is already full.")]
    PoolFull,
    #[msg("Player is not part of the pool.")]
    PlayerNotInPool,
    #[msg("Game pool is not full yet.")]
    PoolNotFull,
    #[msg("Game pool has already been resolved.")]
    PoolAlreadyResolved,
}