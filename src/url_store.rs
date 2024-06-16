use std::collections::HashMap;
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

