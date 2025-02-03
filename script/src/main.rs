use std::env;
use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair};
use solana_sdk::signer::Signer;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Please provide exactly 3 arguments");
        println!("Usage: cargo run <PROGRAM_ID> <PAYER_KEYPAIR_PATH> <RPC_URL>");
        return;
    }

    let program_id = Pubkey::from_str(&args[1])
        .unwrap_or_else(|_| panic!("Could not parse pubkey: {:?}", args[1]));
    let payer_path = &args[2];
    let rpc_url = &args[3];

    let client = RpcClient::new(rpc_url);
    let payer = read_keypair_file(payer_path).expect("Could not read keypair file");
    let message_kp = Keypair::new();
    let blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get latest blockhash");

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
    txn.sign(&[&payer, &message_kp], blockhash);

    let signature = client.send_and_confirm_transaction(&txn).unwrap();
    println!("Transaction confirmed: {:?}", signature);
}
