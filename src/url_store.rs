use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;


#[async_trait]
pub trait UrlStore {
    /* 단축URL에서 (다이제스트, 긴URL) 조회 */
    async fn find_by_short_url(&self, short_url: &str) -> Option<(String, String)>;

    /* URL키 기준에서 짧은URL 조회 */
    async fn find_by_digest(&self, digest: &Vec<u8>) -> Option<String>;

    /*저장하기*/
    async fn save(&mut self, short_url: &str, digest: &Vec<u8>, long_url: &str);
}


pub struct MemoryUrlStore{
    db:HashMap<String, (String, String)>,
    digests:HashMap<String, String>,
}

impl MemoryUrlStore{
    fn new()-> MemoryUrlStore{
        let db = HashMap::new();
        let digests = HashMap::new();
        MemoryUrlStore{db,digests}
    }
}

#[async_trait]
impl UrlStore for MemoryUrlStore{
    async fn find_by_short_url(&self, short_url: &str) -> Option<(String, String)> {
        self.db.get(short_url).cloned()
    }

    async fn find_by_digest(&self, digest: &Vec<u8>) -> Option<String> {
        self.digests.get(&hex::encode(digest)).cloned()
    }

    async fn save(&mut self, short_url: &str, digest: &Vec<u8>, long_url: &str) {
        let digest_hex = hex::encode(digest);
        self.db.insert(String::from(short_url), (digest_hex.clone(), String::from(long_url)));
        self.digests.insert(digest_hex,String::from(short_url));
    }
}

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::types::AttributeValue::{N, S};
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::BackupType::System;
use crate::url;


pub struct DynamoUrlStore{
    client: Client
}

impl DynamoUrlStore{
    pub async fn new()->DynamoUrlStore{
        let regional_provider = RegionProviderChain::default_provider().or_else("ap-northeast-2");
        let config = aws_config::from_env().region(regional_provider).load().await;
        let client = Client::new(&config);

        DynamoUrlStore{
            client
        }
    }
}

const TABLE_NAME:&str = "shorten.url";
const INDEX_NAME:&str = "shorten.url.digest-index";

#[async_trait]
impl UrlStore for DynamoUrlStore {
    async fn find_by_short_url(&self, short_url: &str) -> Option<(String, String)> {
        let request = self
            .client
            .get_item()
            .table_name(TABLE_NAME)
            .key("short_url", S(short_url.to_owned()));

        let result = request.send().await;
        result.ok().and_then(|output| {
            println!("output = {:?}", output);
            let item = output.item()?;
            let digest = item.get("digest")?.as_s().ok()?;
            let long_url = item.get("long_url")?.as_s().ok()?;
            Some((digest.to_owned(), long_url.to_owned()))
        })
    }

    async fn find_by_digest(&self, digest: &Vec<u8>) -> Option<String> {
        let request =self
            .client
            .get_item()
            .table_name(INDEX_NAME)
            .key("digest",S(hex::encode(digest)));

        let result = request.send().await;
        result.ok().and_then(|output| {
            let item = output.item()?.get("short_url")?.as_s().ok()?;
            Some(item.to_owned())
        })
    }

    async fn save(&mut self, short_url: &str, digest: &Vec<u8>, long_url: &str) {
        let timestamp:u128=SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let request= self
            .client
            .put_item()
            .table_name(TABLE_NAME)
            .item("short_URL", S(short_url.to_owned()))
            .item("digest",S(hex::encode(digest)))
            .item("long_url",S(long_url.to_owned()))
            .item("created_at",N(timestamp.to_string()));

        request.send().await.unwrap();
    }
}


pub async fn shorten(store: &mut impl UrlStore, long_url:&str)->String{
    let long_url = long_url.trim();
    let digest = url::digest(long_url);
    let digest_hex = hex::encode(&digest);
    for t in 2..digest.len() {
        let short_url = url::truncate_base32(&digest, t);
        match store.find_by_short(&short_url).await{
            Some((_,hex)) if hex == digest_hex => return short_url,
            _ => {
                let _ = store.save(&short_url,&digest,long_url).await;
                return short_url;
            }
        }
    }
    panic!("endless duplication")

}