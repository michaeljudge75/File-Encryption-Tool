use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use clap::{Parser, Subcommand};
use rand_core::{OsRng};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use file_encryption_tool::main;

fn key_gen() -> String{
    let bits = 2048;
    let rng = [0u8, 16];
    OsRng.try_fill_bytes(&mut rng).unwrap();
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate key");
    let pub_key = RsaPublicKey::from(&priv_key); 
}

fn encrypt(pub_key: &RsaPublicKey, input_path: &str, output_path: &str) -> io::Result<()>{
    let file = File::open(input_path);
    let mut data = Vec::new();
    file.read_to_end(&mut data);
    println!("File size: {}", data.len());
    println!("Encrypting file: {}", input_path);
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data).expect("Failed to encrypt");

    let mut enc_file = OpenOptions::new().write(true).create(true).truncate(true).open(output_path);
    enc_file.write_all(&enc_data);
    println!("File encrypted successfully: {}", enc_file);
    println!("Encrypted file size: {}", enc_file.len());
}
