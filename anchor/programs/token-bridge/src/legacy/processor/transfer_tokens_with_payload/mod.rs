mod native;
mod wrapped;

pub use native::*;
pub use wrapped::*;

use anchor_lang::prelude::*;
use core_bridge_program::types::ExternalAddress;

pub fn new_sender_address(
    sender_authority: &Signer,
    cpi_program_id: Option<Pubkey>,
) -> Result<ExternalAddress> {
    let sender_address = match cpi_program_id {
        Some(program_id) => {
            let (expected_authority, _) = Pubkey::find_program_address(&[b"sender"], &program_id);
            require_eq!(sender_authority.key(), expected_authority);
            program_id
        }
        None => sender_authority.key(),
    };

    Ok(sender_address.into())
}
