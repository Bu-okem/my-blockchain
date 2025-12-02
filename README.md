# Rust Blockchain

This is a simple implementation of a blockchain in Rust, created for educational purposes to understand the basic concepts of blockchain technology.

This project uses Proof of Work (PoW) algorithm to secure the blockchain.

## Features

*   **Block:** Represents a single block in the blockchain with properties like `index`, `timestamp`, `data`, `previous_hash`, `nonce` and `hash`.
*   **Blockchain:** Manages the chain of blocks, including adding new blocks and validating the integrity of the chain.
*   **Hashing:** Uses SHA-256 to calculate the hash of each block, ensuring the immutability of the data.

## Getting Started

### Prerequisites

*   Rust programming language: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/your-username/rust-blockchain.git
    ```
2.  Navigate to the project directory:
    ```bash
    cd rust-blockchain
    ```
3.  Build the project:
    ```bash
    cargo build
    ```

## Usage

The `main` function in `src/main.rs` is currently not set up to run a blockchain simulation. To use the blockchain implementation, you can modify the `main` function to create a new `Blockchain`, add blocks, and validate the chain.

Here's an example of how you could use it:

```rust
fn main() {
    let mut blockchain = Blockchain::new();

    println!("Adding first block...");
    blockchain.add_block("First block after genesis".to_string());

    println!("Adding second block...");
    blockchain.add_block("Second block after genesis".to_string());

    println!("Is chain valid? {}", blockchain.is_chain_valid());

    // You can also inspect the chain
    for block in blockchain.chain {
        println!("Index: {}", block.index);
        println!("Timestamp: {}", block.timestamp);
        println!("Data: {}", block.data);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
        println!("---");
    }
}
```

Then, run the application:
```bash
cargo run
```

This will output the details of each block in the chain and confirm that the chain is valid.
