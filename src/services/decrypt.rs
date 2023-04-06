use std::fs::File;
use std::io::{Error, Read, Write};

use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::{Aead, Payload};
use aes_gcm::aead::consts::U16;

pub fn decrypt() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("./secret.enc")?;
    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)?;

    let key_str = read_key()?;
    let nonce_str = read_nonce()?;

    let nonce = Nonce::<U16>::from_slice(nonce_str.as_slice());
    let cipher = Aes256Gcm::new_from_slice(key_str.as_bytes())?;

    if let Ok(decrypted) = cipher.decrypt(nonce, Payload::from(encrypted_data.as_slice())) {
        let mut output_file = File::create("./secret.dec")?;
        output_file.write_all(&decrypted)?;

        return Ok(());
    }

    Ok(())
}

fn read_key() -> Result<String, Error> {
    let file_path = "./secret.key";
    let file_contents = std::fs::read_to_string(file_path)?;

    Ok(file_contents.chars().take(32).collect())
}

fn read_nonce() -> Result<Vec<u8>, Error> {
    let file_path = "./iv.txt";
    let file_contents = std::fs::read(file_path)?;

    Ok(file_contents)
}
