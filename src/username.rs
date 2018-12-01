use base64::{encode_config, URL_SAFE};
use uuid::Uuid;

pub fn generate_username(value: &str, entropy: &str) -> String {
    let id = Uuid::new_v5(&Uuid::NAMESPACE_URL, format!("{}#{}", value, entropy).as_bytes());
    let username = encode_config(id.as_bytes(), URL_SAFE);
    format!("r--{}", username)
}
