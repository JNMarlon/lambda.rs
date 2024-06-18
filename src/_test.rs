const TABLE_NAME: &str = "shorten.url";
const INDEX_NAME: &str = "shorten.url.digest-index";

#[async_trait]
impl UrlStore for DynamoUrlStore {
    async fn find_by_short(&self, short_url: &str) -> Option<(String, String)> {
        let request = self
            .client
            .get_item()
            .table_name(TABLE_NAME)
            .key("short_url", S(short_url.to_owned()));
        let result = request.send().await;
        result.ok().and_then(|output| {
            println!("output = {:?}", output);
            let attrs = output.item()?;
            let digest = attrs.get("digest")?.as_s().ok()?;
            let long_url = attrs.get("long_url")?.as_s().ok()?;
            Some((digest.to_owned(), long_url.to_owned()))
        })
    }

    async fn find_by_digest(&self, digest: &Vec<u8>) -> Option<String> {
        let request = self
            .client
            .get_item()
            .table_name(INDEX_NAME)
            .key("digest", S(hex::encode(digest)));
        let result = request.send().await;
        result.ok().and_then(|output| {
            let item = output.item()?.get("short_url")?.as_s().ok()?;
            Some(item.to_owned())
        })
    }

    async fn save(&mut self, short_url: &str, digest: &Vec<u8>, long_url: &str) {
        let timestamp: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let request = self
            .client
            .put_item()
            .table_name(TABLE_NAME)
            .item("short_url", S(short_url.to_owned()))
            .item("digest", S(hex::encode(digest)))
            .item("long_url", S(long_url.to_owned()))
            .item("created_at", N(timestamp.to_string()));
        request.send().await.unwrap();
    }
}
