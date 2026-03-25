use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "RustyCrypt", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt { input: PathBuf, output: PathBuf, keyfile: PathBuf },
    Decrypt { input: PathBuf, output: PathBuf, keyfile: PathBuf },
}

fn load_key(path: &PathBuf) -> anyhow::Result<[u8; 32]> {
    let mut f = File::open(path)?;
    let mut buffer = [0u8; 32];
    f.read_exact(&mut buffer).map_err(|_| anyhow::anyhow!("key msut be at least 32 bytes!"))?;
    Ok(buffer)
}

fn create_progress_bar(size: u64, message: &'static str) -> ProgressBar {
    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    pb.set_message(message);
    pb
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encrypt { input, output, keyfile } => {
            let key_data = load_key(keyfile)?;
            let cipher = Aes256Gcm::new_from_slice(&key_data).unwrap();
            
            let data = fs::read(input)?; // Für echtes Streaming bei GB-Dateien müsste man Chacha20-Poly1305 nutzen
            let pb = create_progress_bar(data.len() as u64, "Encrypting");

            let mut nonce_bytes = [0u8; 12];
            OsRng.fill_bytes(&mut nonce_bytes);
            let nonce = Nonce::from_slice(&nonce_bytes);
            
            let ciphertext = cipher.encrypt(nonce, data.as_ref())
                .map_err(|e| anyhow::anyhow!("Error: {}", e))?;
            
            pb.set_position(data.len() as u64);

            let mut final_data = nonce_bytes.to_vec();
            final_data.extend_from_slice(&ciphertext);
            fs::write(output, final_data)?;
            
            pb.finish_with_message("Finished!");
        }
        Commands::Decrypt { input, output, keyfile } => {
            let key_data = load_key(keyfile)?;
            let cipher = Aes256Gcm::new_from_slice(&key_data).unwrap();
            
            let full_data = fs::read(input)?;
            let pb = create_progress_bar(full_data.len() as u64, "Decrypting");

            if full_data.len() < 12 { return Err(anyhow::anyhow!("data to short")); }
            let (nonce_part, ciphertext) = full_data.split_at(12);
            let nonce = Nonce::from_slice(nonce_part);

            let plaintext = cipher.decrypt(nonce, ciphertext)
                .map_err(|e| anyhow::anyhow!("Error: {}", e))?;
            
            pb.set_position(full_data.len() as u64);
            fs::write(output, plaintext)?;
            
            pb.finish_with_message("Decrypted!");
        }
    }
    Ok(())
}