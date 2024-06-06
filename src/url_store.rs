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
