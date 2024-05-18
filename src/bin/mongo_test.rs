use anyhow::Result;
use derive_more::Display;
use futures::TryStreamExt;
use log::error;
use mongodb::bson::Document;
use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
    Client, Database,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::string::ToString;

const MONGODB_CONNECTION_STRING: &str =
    "mongodb+srv://gridlock_mongodb_staging:twx2odQGiqNthtov@cluster0.tmium.mongodb.net/";
const MONGODB_DATABASE_NAME: &str = "gridlockStaging";

async fn get_db() -> Result<Database> {
    let client = Client::with_uri_str(MONGODB_CONNECTION_STRING).await?;
    let db = client.database(&MONGODB_DATABASE_NAME);
    Ok(db)
}

#[tokio::main]
async fn main() -> Result<()> {
    let wallets = get_user_wallets(0, 3000).await?;
    dbg!(wallets.len());
    dbg!(wallets.iter().filter(|w| w.wallet_owner.len() == 1).count());
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletData {
    associated_guardians: Vec<AssociatedGuardian>,
    #[serde(rename = "coinType")]
    #[serde(deserialize_with = "deserialize_coin_kind")]
    kind: CoinKind,
    wallet_owner: Vec<WalletOwner>,
    key_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletOwner {
    node_id: String,
    node_pool: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    #[serde(rename = "type")]
    kind: NodeKind,
    node_id: Option<String>,
    public_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NodeKind {
    #[serde(rename = "ownerGuardian")]
    OwnerGuardian,
    #[serde(rename = "gridlockGuardian")]
    GridlockGuardian,
    #[serde(rename = "partnerGuardian")]
    PartnerGuardian,
    #[serde(rename = "userGuardian")]
    UserGuardian,
}

#[derive(Serialize, Debug)]
pub enum CoinKind {
    #[serde(rename = "solana")]
    Solana,
    #[serde(rename = "ethereum")]
    Ethereum,
    #[serde(rename = "2fa")]
    TwoFa,
    #[serde(rename = "sr25519")]
    Sr25519,
}

fn deserialize_coin_kind<'de, D>(deserializer: D) -> Result<CoinKind, D::Error>
where
    D: Deserializer<'de>,
{
    let coin_kind_str = String::deserialize(deserializer)?;
    let coin_kind = match coin_kind_str.as_str() {
        "solana" | "polkadot" | "kusama" | "astar" => CoinKind::Solana,
        "2fa" => CoinKind::TwoFa,
        "sr25519" | "polkadot:sr" => CoinKind::Sr25519,
        // NOTE: all other coins should be erc20 based
        _ => CoinKind::Ethereum,
    };
    Ok(coin_kind)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssociatedGuardian {
    node_id: Option<String>,
    index: Option<u32>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Display, PartialEq)]
pub struct NodeId(String);

async fn get_user_wallets(skip: u32, limit: u32) -> Result<Vec<WalletData>> {
    let db = get_db().await?;
    let pipeline = [
        doc! {
            "$lookup": {
                "from": "users",
                "let": {
                   "walletsUsersId": { "$toObjectId": "$userId" }
                 },
                 "pipeline": [
                   { "$match": { "$expr": { "$eq" : [ "$_id", "$$walletsUsersId" ] } } },
                 ],
                "as": "walletOwner",
            }
        },
        doc! {
            "$project": {
                "associatedGuardians": {
                    "nodeId": 1,
                    "index": 1,
                },
                "coinType": 1,
                "keyId": 1,
                "walletOwner": {
                    "nodePool": {
                        "type": 1,
                        "nodeId": 1,
                        "publicKey": 1,
                    },
                    "nodeId": 1,
                }
            }
        },
        doc! {
        "$skip": skip
        },
        doc! {
          "$limit": limit
        },
    ];
    let wallets = db.collection::<Document>("wallets");
    let cursor = wallets.aggregate(pipeline, None).await?;

    let wallets: Vec<_> = cursor.try_collect().await?;
    let wallet_data = wallets
        .into_iter()
        .map(|d| {
            let str = serde_json::to_string_pretty(&d)?;
            let x: WalletData = serde_json::from_str(&str)?;
            Result::<WalletData, serde_json::Error>::Ok(x)
        })
        .inspect(|e| {
            if let Err(e) = e {
                println!("Failed convert users and wallets collections to walletOwner: {e}")
            }
        })
        .flatten()
        .collect();

    Ok(wallet_data)
}
