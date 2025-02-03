use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::{msg, system_instruction, system_program};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MessageAccount {
    pub message: [u8; 11],
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    if accounts.len() != 3 {
        msg!(
            "Expected 3 account keys but {} were provided",
            accounts.len()
        );
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let signer = &accounts[0];
    let message_account = &accounts[1];
    let system_program = &accounts[2];
    if !signer.is_signer || !signer.is_writable {
        msg!("{:?} must be signer and writeable", signer.key);
        return Err(ProgramError::InvalidAccountData);
    }
    if !message_account.is_signer || !message_account.is_writable {
        msg!("{:?} must be signer and writeable", message_account.key);
        return Err(ProgramError::InvalidAccountData);
    }
    if system_program.key != &system_program::id() {
        msg!("{:?} must be system program", system_program.key);
        return Err(ProgramError::InvalidAccountData);
    }

    let message_account_size = std::mem::size_of::<MessageAccount>();
    invoke(
        &system_instruction::create_account(
            signer.key,
            message_account.key,
            Rent::default().minimum_balance(message_account_size),
            message_account_size as u64,
            program_id,
        ),
        accounts,
    )?;

    let mut message_data = message_account.try_borrow_mut_data()?;
    message_data.copy_from_slice(&[72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]);

    Ok(())
}

#[cfg(test)]
mod test {
    use solana_program::instruction::{AccountMeta, Instruction};
    use solana_program_test::*;
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::signature::Keypair;
    use solana_sdk::signer::Signer;
    use solana_sdk::system_program;
    use solana_sdk::transaction::Transaction;

    use crate::process_instruction;

    #[tokio::test]
    async fn test_example() {
        let program_id = Pubkey::new_unique();
        let message_kp = Keypair::new();

        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "rpcx_hello_world_program",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        let instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(message_kp.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data: vec![],
        };

        let mut txn = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        txn.sign(&[&payer, &message_kp], recent_blockhash);
        banks_client.process_transaction(txn).await.unwrap();
    }
}
