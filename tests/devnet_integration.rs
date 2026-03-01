//! Devnet integration test for Solaudit.
//!
//! Required env vars (in .env or exported in the shell):
//!   SOLAUDIT_DEVNET_TEST=1
//!   COUNTER_PROGRAM_ID=<deployed program pubkey>
//!
//!   cargo test --test devnet_integration -- --nocapture

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use solana_system_interface::instruction as system_instruction;
use solaudit::{
    analysis::engine::{analyse, RetrySafety},
    models::types::AccountSnapshot,
    rpc::client::SolanaRpc,
};

const DEVNET_RPC: &str = "https://api.devnet.solana.com";
const COUNTER_SPACE: usize = 8;

fn load_payer() -> Keypair {
    let path = std::env::var("SOLAUDIT_TEST_KEYPAIR_PATH").unwrap_or_else(|_| {
        let home = std::env::var("HOME").expect("HOME env var not set");
        format!("{}/.config/solana/id.json", home)
    });
    read_keypair_file(&path)
        .unwrap_or_else(|e| panic!("Failed to load keypair from {}: {}", path, e))
}

#[tokio::test]
async fn test_devnet_counter_increment_diff() {
    dotenvy::dotenv().ok();

    if std::env::var("SOLAUDIT_DEVNET_TEST").is_err() {
        println!(
            "SKIP: set SOLAUDIT_DEVNET_TEST=1 and COUNTER_PROGRAM_ID=<pubkey> to run this test."
        );
        return;
    }

    let program_id: Pubkey = std::env::var("COUNTER_PROGRAM_ID")
        .expect("COUNTER_PROGRAM_ID must be set")
        .parse()
        .expect("COUNTER_PROGRAM_ID is not a valid pubkey");

    let payer = load_payer();
    let counter_kp = Keypair::new();
    let counter_pk = counter_kp.pubkey();

    println!("Payer:   {}", payer.pubkey());
    println!("Counter: {}", counter_pk);
    println!("Program: {}", program_id);

    let raw = RpcClient::new_with_commitment(
        DEVNET_RPC.to_string(),
        CommitmentConfig::confirmed(),
    );

    let version = raw
        .get_version()
        .await
        .expect("Devnet RPC is unreachable — make sure you are online and devnet is up");
    println!("Devnet version: {}", version.solana_core);

    let balance = raw
        .get_balance(&payer.pubkey())
        .await
        .expect("Failed to fetch payer balance");
    assert!(
        balance >= 1_000_000,
        "Payer has insufficient funds ({} lamports). Run: solana airdrop 1 --url devnet",
        balance
    );

    let rent = raw
        .get_minimum_balance_for_rent_exemption(COUNTER_SPACE)
        .await
        .expect("Failed to fetch rent-exemption minimum");

    let solaudit = SolanaRpc::new("devnet").expect("Failed to build SolanaRpc");

    let before_tx1 = solaudit
        .fetch_snapshot_or_default(&counter_pk.to_string())
        .await
        .expect("fetch_snapshot_or_default failed");

    assert_eq!(
        before_tx1.data_len, 0,
        "Counter account already exists — use a fresh keypair"
    );
    println!("Pre-state  data_len: {}", before_tx1.data_len);

    let blockhash1 = raw
        .get_latest_blockhash()
        .await
        .expect("Failed to fetch recent blockhash for TX1");

    let create_ix = system_instruction::create_account(
        &payer.pubkey(),
        &counter_pk,
        rent,
        COUNTER_SPACE as u64,
        &program_id,
    );

    let increment_ix = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(counter_pk, false)],
        data: vec![],
    };

    let tx1 = Transaction::new_signed_with_payer(
        &[create_ix, increment_ix],
        Some(&payer.pubkey()),
        &[&payer, &counter_kp],
        blockhash1,
    );

    let tx1_bytes = bincode::serialize(&tx1).expect("Failed to serialize TX1");
    let tx1_b64 = STANDARD.encode(&tx1_bytes);

    let sim1 = solaudit
        .simulate_transaction(&tx1_b64, &counter_pk.to_string())
        .await
        .expect("simulate_transaction for TX1 failed");

    if let Some(ref err) = sim1.error {
        panic!("TX1 simulation returned an error: {}", err);
    }

    println!("TX1 simulation logs:");
    for log in &sim1.logs {
        println!("  {}", log);
    }

    let after_tx1 = sim1
        .post_snapshot
        .expect("TX1 simulation returned no post-snapshot");

    println!("Post-state (TX1) data_len: {}", after_tx1.data_len);

    let result1 = analyse(before_tx1, after_tx1, sim1.logs);

    assert!(result1.diff.data_len_changed, "data_len_changed");

    let sig1 = raw
        .send_and_confirm_transaction(&tx1)
        .await
        .expect("Failed to submit TX1 to devnet");
    println!("TX1 confirmed: {}", sig1);

    let before_tx2 = {
        let mut retries = 10u32;
        loop {
            match raw.get_account(&counter_pk).await {
                Ok(account) => {
                    break AccountSnapshot {
                        pubkey: counter_pk,
                        lamports: account.lamports,
                        owner: account.owner,
                        executable: account.executable,
                        data_len: account.data.len(),
                        data: account.data,
                        rent_epoch: account.rent_epoch,
                    };
                }
                Err(_) if retries > 0 => {
                    retries -= 1;
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
                Err(e) => panic!("Counter account not visible after retries: {}", e),
            }
        }
    };

    println!("Before TX2: data={:?}", before_tx2.data);

    let blockhash2 = raw
        .get_latest_blockhash()
        .await
        .expect("Failed to fetch recent blockhash for TX2");

    let increment_ix2 = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(counter_pk, false)],
        data: vec![],
    };

    let tx2 = Transaction::new_signed_with_payer(
        &[increment_ix2],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash2,
    );

    let tx2_bytes = bincode::serialize(&tx2).expect("Failed to serialize TX2");
    let tx2_b64 = STANDARD.encode(&tx2_bytes);

    let sim2 = solaudit
        .simulate_transaction(&tx2_b64, &counter_pk.to_string())
        .await
        .expect("simulate_transaction for TX2 failed");

    if let Some(ref err) = sim2.error {
        panic!("TX2 simulation returned an error: {}", err);
    }

    println!("TX2 simulation logs:");
    for log in &sim2.logs {
        println!("  {}", log);
    }

    let after_tx2 = sim2
        .post_snapshot
        .expect("TX2 simulation returned no post-snapshot");

    println!("After TX2:  data={:?}", after_tx2.data);

    let result2 = analyse(before_tx2, after_tx2, sim2.logs);

    assert!(result2.diff.data_changed, "data_changed");
    assert!(matches!(result2.classification.safety, RetrySafety::Unsafe));
    assert!(result2
        .classification
        .reasons
        .contains(&"Account data content changed".to_string()));
}
