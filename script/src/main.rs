use std::str::FromStr;

use clap::{command, Parser};

use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair};
use solana_sdk::signer::Signer;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The program ID to interact with
    #[arg(long)]
    program_id: String,

    /// Path to the payer's keypair file. If not provided, uses default Solana CLI keypair
    #[arg(short, long)]
    payer: Option<String>,

    /// RPC URL to connect to
    #[arg(short, long, default_value = "https://testnet.atlas.xyz")]
    rpc_url: String,
}

fn main() {
    let args = Args::parse();

    let program_id = Pubkey::from_str(&args.program_id)
        .unwrap_or_else(|_| panic!("Could not parse pubkey: {:?}", args.program_id));

    let payer = match args.payer {
        Some(path) => read_keypair_file(&path).expect("Could not read keypair file"),
        None => read_keypair_file(&*solana_cli_config::Config::default().keypair_path)
            .expect("Could not read default keypair file"),
    };

    let client = RpcClient::new(args.rpc_url);
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
