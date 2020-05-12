use rand::RngCore;

pub fn generate() -> String {
    let mut token_bytes = [0; 18];
    rand::thread_rng().fill_bytes(&mut token_bytes);
    base64::encode_config(token_bytes, base64::URL_SAFE_NO_PAD)
}
