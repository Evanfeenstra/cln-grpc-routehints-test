mod cln;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let creds_dir = "../cln/creds2";
    let creds = utils::collect_creds(creds_dir).await?;
    let port = "10010";
    let mut client = cln::ClnRPC::try_new(port, &creds, 50).await?;

    let info = client.get_info().await?;
    println!("INFO: {:?}", info);

    let pk = "025526f10198f5004f8985ffd16e9660296c0339f8bedc1597c8026b8b65c84401";
    let amt = 500;
    let lhpk = "02c7046d20f62012362ccf835fe5b4d4a1708e518592f216afeefabeadfc20154b";
    let scid = "1x5x1";

    let hm = client.keysend_with_route_hint(pk, amt, lhpk, scid).await?;
    println!("TRY: {:?}", hm);

    Ok(())
}
