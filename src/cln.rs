use crate::utils::*;
use anyhow::{anyhow, Result};
use cln_grpc::pb;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

pub struct ClnRPC {
    pub client: pb::node_client::NodeClient<Channel>,
}

impl ClnRPC {
    // try new a few times
    pub async fn try_new(grpc_port: &str, creds: &Creds, i: usize) -> Result<Self> {
        for iteration in 0..i {
            if let Ok(c) = Self::new(grpc_port, creds).await {
                return Ok(c);
            }
            sleep_ms(1000).await;
            println!("retry CLN connect {}", iteration);
        }
        Err(anyhow!("could not connect to CLN"))
    }
    pub async fn new(grpc_port: &str, creds: &Creds) -> Result<Self> {
        // println!("CA PEM {:?}", &creds.ca_pem);
        // println!("CLEINT PEM {:?}", &creds.client_pem);
        // println!("CLIENT KEY {:?}", &creds.client_key);

        let ca = Certificate::from_pem(&creds.ca_pem);
        let ident = Identity::from_pem(&creds.client_pem, &creds.client_key);

        let tls = ClientTlsConfig::new()
            .domain_name("cln")
            .identity(ident)
            .ca_certificate(ca);

        let url = format!("http://[::1]:{}", grpc_port);
        let channel = Channel::from_shared(url)?
            .tls_config(tls)?
            .connect()
            .await?;
        let client = pb::node_client::NodeClient::new(channel);

        Ok(Self { client })
    }

    pub async fn get_info(&mut self) -> Result<pb::GetinfoResponse> {
        let response = self.client.getinfo(pb::GetinfoRequest {}).await?;
        Ok(response.into_inner())
    }

    pub async fn keysend_with_route_hint(
        &mut self,
        id: &str,
        amt: u64,
        last_hop_id: &str,
        scid: &str,
    ) -> Result<pb::KeysendResponse> {
        let id = hex::decode(id)?;
        let mut routehints = pb::RoutehintList { hints: vec![] };
        let mut hint1 = pb::Routehint { hops: vec![] };
        let hop1 = pb::RouteHop {
            id: hex::decode(last_hop_id)?,
            short_channel_id: scid.to_string(),
            feebase: Some(_amount(1000)),
            expirydelta: 40,
            feeprop: 1,
        };
        hint1.hops.push(hop1);
        routehints.hints.push(hint1);
        let response = self
            .client
            .key_send(pb::KeysendRequest {
                destination: id,
                amount_msat: Some(_amount(amt)),
                routehints: Some(routehints),
                ..Default::default()
            })
            .await?;
        Ok(response.into_inner())
    }
}

fn _amount_or_any(msat: u64) -> Option<pb::AmountOrAny> {
    Some(pb::AmountOrAny {
        value: Some(pb::amount_or_any::Value::Amount(_amount(msat))),
    })
}
fn _amount_or_all(msat: u64) -> Option<pb::AmountOrAll> {
    Some(pb::AmountOrAll {
        value: Some(pb::amount_or_all::Value::Amount(_amount(msat))),
    })
}
fn _amount(msat: u64) -> pb::Amount {
    pb::Amount { msat }
}
