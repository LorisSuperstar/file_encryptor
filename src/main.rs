mod args;
use std::{fs::{self, OpenOptions}, io::stdin};
use args::FileEncryptorArgs;
use clap::Parser;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};
use std::fs::File;
use std::io::prelude::*;
use std::io;

fn main() -> std::io::Result<()> {

    println!("do you want to encrypt or decrypt? (e / d)");
    let stdin = io::stdin();
    let mut user_descision = String::new();
    stdin.read_line(&mut user_descision)?;

    if user_descision.trim() == "e" {

    let arg = FileEncryptorArgs::parse();

    let content = fs::read_to_string(&arg.file)?;
    let key = Aes256Gcm::generate_key(OsRng);


    let mut file = File::create("Key.txt")?;
    file.write_all(&key)?;
    println!("written key to Key.txt");

    let mut file = OpenOptions::new().create(true).truncate( true).write(true).open(arg.file).unwrap();
    file.write_all(&encrypt(&key, content.as_bytes())).unwrap();
    }
    Ok(())
}


fn encrypt(key: &Key<Aes256Gcm>, plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per encryption

    let mut ciphertext = cipher.encrypt(&nonce, plaintext)
        .expect("encryption failure!"); // In production, handle this Result

    // Prepend nonce to ciphertext so you can retrieve it during decryption
    let mut final_data = nonce.to_vec();
    final_data.append(&mut ciphertext);
    
    final_data
}
