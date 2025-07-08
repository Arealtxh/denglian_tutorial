use openssl::rsa::Rsa;
use openssl::pkey::Private;
use openssl::sign::{Signer, Verifier};
use openssl::pkey::PKey;
use sha2::{Sha256, Digest};
use rand::Rng;
use hex;

fn main() {
    // 1. 生成RSA密钥对
    let rsa = Rsa::generate(2048).expect("生成密钥对失败");
    let private_key = rsa.private_key_to_pem().unwrap();
    let public_key = rsa.public_key_to_pem().unwrap();
    println!("私钥(PEM):\n{}", String::from_utf8_lossy(&private_key));
    println!("公钥(PEM):\n{}", String::from_utf8_lossy(&public_key));

    // 2. POW: 寻找sha256("txh"+nonce)以4个0开头
    let nickname = "txh";
    let mut nonce = 0u64;
    let pow_result;
    loop {
        let input = format!("{}{}", nickname, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();
        let hash_hex = hex::encode(&hash);
        if hash_hex.starts_with("0000") {
            pow_result = input;
            println!("找到POW! nonce = {}", nonce);
            println!("POW字符串: {}", pow_result);
            println!("哈希值: {}", hash_hex);
            break;
        }
        nonce += 1;
    }

    // 3. 用私钥签名
    let key = Rsa::private_key_from_pem(&private_key).unwrap();
    let pkey = PKey::from_rsa(key).unwrap();
    let mut signer = Signer::new_without_digest(&pkey).unwrap();
    signer.update(pow_result.as_bytes()).unwrap();
    let signature = signer.sign_to_vec().unwrap();
    println!("签名(base64): {}", base64::encode(&signature));

    // 4. 用公钥验证
    let pubkey = Rsa::public_key_from_pem(&public_key).unwrap();
    let pub_pkey = PKey::from_rsa(pubkey).unwrap();
    let mut verifier = Verifier::new_without_digest(&pub_pkey).unwrap();
    verifier.update(pow_result.as_bytes()).unwrap();
    let is_valid = verifier.verify(&signature).unwrap();
    println!("签名验证结果: {}", is_valid);
}
