extern crate solana_sdk;

use solana_sdk::{
    account_info::AccountInfo,
    account_info::AccountInfoMut,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

// The NFT smart contract's program ID
const PROGRAM_ID: &str = "NFT_PROGRAM_ID";

// The number of words in the NFT data
const NFT_DATA_WORDS: usize = 4;

// The NFT smart contract
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), String> {
    let mut account_metas = Vec::new();
    for i in 0..accounts.len() {
        let a = &accounts[i];
        account_metas.push(AccountMeta::new(a.is_signer, a.is_writable));
    }
    let instruction = Instruction::new(account_metas, instruction_data);

    match instruction.program_id {
        // Issue a new NFT
        key if key == *_program_id => {
            let mut nft_account = match accounts[0].try_account_ref_mut() {
                Ok(account) => account,
                Err(_) => return Err("Error accessing account data".to_string()),
            };

            // Check that the NFT account is empty
            if nft_account.len() != 0 {
                return Err("Error: NFT account is not empty".to_string());
            }

            // Set the NFT data
            let nft_data: Vec<u64> = instruction.data[..NFT_DATA_WORDS]
                .iter()
                .map(|word| word.to_le_bytes())
                .flatten()
                .map(|byte| u64::from(byte))
                .collect();
            nft_account[..NFT_DATA_WORDS].copy_from_slice(&nft_data[..]);
            Ok(())
        }
        // Transfer an NFT
        _ => {
            let from_account = match accounts[0].try_account_ref_mut() {
                Ok(account) => account,
                Err(_) => return Err("Error accessing 'from' account data".to_string()),
            };
            let
