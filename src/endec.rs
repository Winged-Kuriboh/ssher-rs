use base64::{Engine, engine::general_purpose};

pub(crate) fn encode_string(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        // base64 encode
        Some(general_purpose::STANDARD.encode(s))
    }
}

pub(crate) fn decode_string(s: &String) -> anyhow::Result<String> {
    if s.is_empty() {
        anyhow::bail!("string is empty")
    } else {
        // base64 decode
        if let Ok(b) = general_purpose::STANDARD.decode(s) {
            Ok(String::from_utf8(b)?)
        } else {
            anyhow::bail!("string decode failed")
        }
    }
}
