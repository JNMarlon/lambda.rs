use sha2::digest::Update;
use sha2::{Digest, Sha256};

/**주어진 문자열로 부터 sha256 다이제스트 생성. 256bit, 그러니까 32바이트 벡터 반환됨*/
pub fn digest(url:&str)-> Vec<u8>{
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    hasher.finalize().to_vec()
}


