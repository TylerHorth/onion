use anyhow::Result;
use openssl::aes::AesKey;
use openssl::symm::Cipher;
use std::convert::TryInto;

pub fn aes_cbc(input: Vec<u8>) -> Result<Vec<u8>> {
    let kek = AesKey::new_decrypt(&input[..32]).unwrap();
    let iv = input[32..40].try_into()?;
    let encrypted_key = &input[40..80];
    let payload_iv = &input[80..96];
    let payload = &input[96..];

    let mut key = [0; 32];
    openssl::aes::unwrap_key(&kek, Some(iv), &mut key, encrypted_key).unwrap();

    let cipher = Cipher::aes_256_cbc();
    let result = openssl::symm::decrypt(cipher, &key, Some(payload_iv), payload).unwrap();

    Ok(result)
}
