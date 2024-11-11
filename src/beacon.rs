#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize)]
struct BeaconApiResponse<T> {
    data: T,
}

#[derive(Deserialize)]
struct Header {
    header: HeaderInner,
}

#[derive(Deserialize)]
struct HeaderInner {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    #[serde(with = "quoted_u64")]
    slot: u64,
}

// serde
pub mod quoted_u64 {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(x: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&x.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u64>().map_err(serde::de::Error::custom)
    }
}

pub struct BeaconClient {
    base_url: reqwest::Url,
    client: reqwest::Client,
}

impl BeaconClient {
    pub fn new(base_url: reqwest::Url) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn head_slot(&self) -> eyre::Result<u64> {
        let headers_url = self.base_url.join("eth/v1/beacon/headers/head")?;
        let header = self
            .client
            .get(headers_url)
            .send()
            .await?
            .json::<BeaconApiResponse<Header>>()
            .await?
            .data;
        let head_slot = header.header.message.slot;
        Ok(head_slot)
    }
}
