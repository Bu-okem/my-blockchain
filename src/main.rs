use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Hello, world!");
}

struct Block {
    index: u64,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn joined_values(&self) -> String {
        format!(
            "{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash
        )
    }

    fn calculate_hash(&self) -> String {
        let string_data = self.joined_values();
        let mut hasher = Sha256::new();
        hasher.update(string_data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let mut genesis_block = Block {
            index: 0,
            timestamp: "0".to_string(),
            previous_hash: "0".to_string(),
            hash: "".to_string(),
            data: "genesis".to_string(),
        };
        genesis_block.hash = genesis_block.calculate_hash();

        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) -> () {
        let last_block_hash = self.chain.last();
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let mut new_block = Block {
            index: self.chain.len() as u64,
            timestamp: since_the_epoch.as_millis().to_string(),
            previous_hash: match last_block_hash {
                Some(value) => value.hash.to_string(),
                None => "".to_string(),
            },
            data: data,
            hash: String::from(""),
        };

        new_block.hash = new_block.calculate_hash();

        self.chain.push(new_block);
    }

    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash || current.calculate_hash() != current.hash {
                return false;
            }
        }
        true
    }
}
