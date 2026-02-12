mod args;
use std::{fs, io::stdin};
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
    println!("written key to Key.txt")
    }

    Ok(())
}
