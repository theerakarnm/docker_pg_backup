use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::Write;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

/// Encrypts the file at `input_file` using AES256-CBC encryption.
/// Returns the path to the encrypted file.
pub fn encrypt_file(input_file: &str, encryption_key: &[u8]) -> Result<String, Box<dyn Error>> {
    // Generate a random 16-byte IV
    let iv: [u8; 16] = rand::thread_rng().gen();

    // Read the plaintext file contents
    let plaintext = std::fs::read(input_file)?;

    // Create the cipher instance
    let cipher = Aes256Cbc::new_from_slices(encryption_key, &iv)
        .map_err(|e| format!("Failed to create cipher instance: {:?}", e))?;
    let ciphertext = cipher.encrypt_vec(&plaintext);

    // Write the encrypted file, prepending the IV
    let encrypted_file = format!("{}.enc", input_file);
    let mut file = File::create(&encrypted_file)?;
    file.write_all(&iv)?; // Write IV first
    file.write_all(&ciphertext)?; // Then write ciphertext

    Ok(encrypted_file)
}
