//! Program entrypoint

#![cfg(all(target_os = "lumos", not(feature = "no-entrypoint")))]

use {
    crate::{error::SinglePoolError, processor::Processor},
    lumos_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program_error::PrintProgramError,
        pubkey::Pubkey,
    },
    lumos_security_txt::security_txt,
};

lumos_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<SinglePoolError>();
        Err(error)
    } else {
        Ok(())
    }
}

security_txt! {
    // Required fields
    name: "SPL Single-Validator Stake Pool",
    project_url: "https://spl.lumos.com/single-pool",
    contacts: "link:https://github.com/lumos-labs/lumos-program-library/security/advisories/new,mailto:security@lumos.com,discord:https://discord.gg/lumos",
    policy: "https://github.com/lumos-labs/lumos-program-library/blob/master/SECURITY.md",

    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/lumos-labs/lumos-program-library/tree/master/single-pool/program",
    source_revision: "ef44df985e76a697ee9a8aabb3a223610e4cf1dc",
    source_release: "single-pool-v1.0.0",
    auditors: "https://github.com/lumos-labs/security-audits#single-stake-pool"
}
