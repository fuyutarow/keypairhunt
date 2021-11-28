use ed25519_dalek::{ExpandedSecretKey, SecretKey};
use serde::{Deserialize, Serialize};
use solana_sdk::signer::keypair::Keypair;
use std::{io::prelude::*, sync::Arc};
use tokio::{io::AsyncWriteExt, sync::Mutex};

use serde_big_array::big_array;
big_array! { BigArray;}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Data {
    address: String,
    #[serde(with = "BigArray")]
    secret_key: [u8; 64],
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let count = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let cnt = Arc::clone(&count);

        tokio::spawn(async move {
            let mut count = cnt.lock().await;
            loop {
                let keypair = Keypair::new();
                let address = keypair.to_base58_string();

                if address.starts_with("Host") || address.to_lowercase().starts_with("xhost") {
                    let data = Data {
                        address,
                        secret_key: ExpandedSecretKey::from(keypair.secret()).to_bytes(),
                    };
                    let s = serde_json::to_string(&data).expect("serialize json");

                    let mut file = tokio::fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open("keypairs.jsonl")
                        .await
                        .expect("open file");
                    file.write(format!("{}\n", &s).as_bytes())
                        .await
                        .expect("write file");
                    // *count += 1;
                    println!("{}", &s);
                }
            }
        });
    }

    loop {
        if *count.lock().await > 100 {
            break;
        }
    }

    Ok(())
}
