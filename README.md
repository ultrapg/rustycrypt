# RustyCrypt
*RustyCrypt is a lightweight, high-performance command-line tool written in Rust for securing your files. It utilizes the industry-standard AES-256-GCM authenticated encryption to ensure your data remains both private and untampered.*


### Features

  - Secure Encryption: Uses AES-256-GCM (Galois/Counter Mode) for high-speed, authenticated encryption.

  - Progress Tracking: Integrated progress bars via indicatif to keep you informed during processing.

  - Simple CLI: Intuitive commands for encryption and decryption powered by clap.

  - Safety First: Built with Rust’s memory safety guarantees.
---
### Installation
**download latest release:**

https://github.com/ultrapg/rustycrypt/releases/tag/v1.0.0

**To build RustyCrypt from source:**

  Clone the repository:
  ```
  git clone https://github.com/yourusername/rustycrypt.git
  cd rustycrypt
  ```
  Build the release version:
  ```
  cargo build --release
  ```
---
### Usage

RustyCrypt requires a at least 32bytes key file for operations. You can generate one using openssl or any secure random generator
Encrypt a File
```
./rustycrypt encrypt --input secret.txt --output secret.enc --keyfile my.key
```
Decrypt a File
```
./rustycrypt decrypt --input secret.enc --output restored.txt --keyfile my.key
```
---
### How it Works

  - Key Loading: The tool reads exactly 32 bytes from your specified key file.

  - Nonce Generation: For every encryption, a unique 12-byte random Nonce (Number used once) is generated using OsRng.

  - Authentication: AES-GCM ensures that if the encrypted file is modified by even a single bit, decryption will fail, protecting you against tampering.

  - Storage: The 12-byte Nonce is prepended to the ciphertext in the output file, making it easy to manage.
---
### Technical Details

  Language: Rust (Edition 2024)

  Crates Used:

  - aes-gcm: AEAD encryption implementation.

  - clap: Command-line argument parsing.

  - indicatif: Terminal progress reporting.

  - anyhow: Flexible error handling.
