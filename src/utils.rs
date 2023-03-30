use anyhow::Result;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, Error};

async fn reader(filename: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(filename).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    Ok(contents)
}

pub struct Creds {
    pub ca_pem: Vec<u8>,
    pub client_pem: Vec<u8>,
    pub client_key: Vec<u8>,
}
pub async fn collect_creds(root: &str) -> Result<Creds> {
    let ca_pem = reader(&format!("{}/ca.pem", root)).await?;
    let client_pem = reader(&format!("{}/client.pem", root)).await?;
    let client_key = reader(&format!("{}/client-key.pem", root)).await?;

    Ok(Creds {
        ca_pem,
        client_pem,
        client_key,
    })
}

pub async fn sleep_ms(n: u64) {
    tokio::time::sleep(std::time::Duration::from_millis(n)).await;
}
