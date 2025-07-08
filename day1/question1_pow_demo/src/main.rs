use sha2::{Sha256, Digest};
use std::time::Instant;

fn pow(prefix_zeros: usize, nickname: &str) {
    let mut nonce = 0u64;
    let prefix = "0".repeat(prefix_zeros);
    let start = Instant::now();

    loop {
        let content = format!("{}{}", nickname, nonce);
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let hash = hasher.finalize();
        let hash_hex = hex::encode(&hash);

        if hash_hex.starts_with(&prefix) {
            let duration = start.elapsed();
            println!(
                "满足 {} 个 0，耗时：{:?}\n内容：{}\nHash值：{}\n",
                prefix_zeros, duration, content, hash_hex
            );
            break;
        }
        nonce += 1;
    }
}

fn main() {
    let nickname = "txh"; // 你的昵称
    pow(4, nickname);
    pow(5, nickname);
}
