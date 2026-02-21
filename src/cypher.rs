use base64::{Engine as _, engine::general_purpose};
use simple_encrypt::{decrypt_string, encrypt_string};

pub fn normalize_key(code: &str) -> Vec<u8> {
    let mut vec = code.as_bytes().to_vec();
    vec.resize(32, 0u8);
    vec
}

pub fn cypher(txt: &String, code: &str) -> Result<String, String> {
    Ok(general_purpose::STANDARD.encode(
        encrypt_string(txt, &normalize_key(code))
            .map_err(|e| format!("Could not encrypt data: {e:#}"))?,
    ))
}

pub fn uncypher(txt: String, code: &str) -> Result<String, String> {
    decrypt_string(
        &general_purpose::STANDARD
            .decode(txt.clone())
            .map_err(|e| format!("Could not decode base64({txt}): {e:#}"))?,
        &normalize_key(code),
    )
    .map_err(|e| format!("Could  not decrypt data: {e:#}"))
}
