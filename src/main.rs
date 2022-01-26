use serde::{Deserialize, Serialize};
use solana_sdk::signer::{keypair::Keypair, Signer};
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
                let address = keypair.pubkey().to_string();

                // if address.to_lowercase().starts_with("h3sw") {
                if address.to_lowercase().starts_with("xhost")
                    || address.to_lowercase().starts_with("fuyuta")
                    || address.to_lowercase().starts_with("sktnky")
                    || address.to_lowercase().starts_with("poker")
                    || address.to_lowercase().starts_with("t3auth")
                    || address.starts_with("Tip3")
                    || address.starts_with("fuyu")
                    || address.starts_with("Host3")
                    || address.starts_with("Alice")
                    || address.starts_with("Charl")
                {
                    let data = Data {
                        address,
                        secret_key: keypair.to_bytes(),
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
                    *count += 1;
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
