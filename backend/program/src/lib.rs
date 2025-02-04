use anchor_lang::prelude::*;
use code_vm_api::prelude::*;
use solana_include_idl::{include_idl, parse::IdlType};
use steel::*;

mod opcode;
mod instruction;
use instruction::*;

// Include the compressed IDL in an ELF section on the program binary.
include_idl!(IdlType::Anchor, concat!(env!("OUT_DIR"), "/idl.zip"));

#[program]
pub mod code_vm {
    use super::*;
    // Import the account types for game pool instructions.
    use crate::instruction::{JoinPool, SubmitScore, ResolvePool};

    pub fn init_vm(ctx: Context<InitVm>, data: InitVmArgs) -> anchor_lang::Result<()> {
        process_init_vm(ctx.accounts, data)
    }

    // Other instruction handlers (init_memory, init_storage, etc.) can be defined here.

    pub fn join_pool(ctx: Context<JoinPool>, fee: u64) -> anchor_lang::Result<()> {
        crate::instruction::join_pool(ctx, fee)
    }

    pub fn submit_score(ctx: Context<SubmitScore>, score: u64) -> anchor_lang::Result<()> {
        crate::instruction::submit_score(ctx, score)
    }

    pub fn resolve_pool(ctx: Context<ResolvePool>) -> anchor_lang::Result<()> {
        crate::instruction::resolve_pool(ctx)
    }
}