

#[cfg(test)]
mod tests {
    use crate::url::digest;
    #[test]
    fn test_digest(){
        let long_url_1 = "https://jinArchive.com/long-url-1";
        let digest_1 = digest(long_url_1);

        assert_eq!(32,digest_1.len())
    }

}
