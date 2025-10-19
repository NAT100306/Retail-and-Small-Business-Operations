use crate::models::{Block, Transaction};
use chrono::Utc;
use sha2::{Sha256, Digest};
use serde_json;
use hex;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    difficulty: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Self {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty: 2,
        };

        // Create genesis block
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now(),
            transactions: Vec::new(),
            previous_hash: String::from("0"),
            hash: String::new(),
            nonce: 0,
        };

        let hash = self.calculate_hash(&genesis_block);
        let mut genesis_block = genesis_block;
        genesis_block.hash = hash;
        
        self.chain.push(genesis_block);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
        println!("📝 Transaction added to pending pool");
    }

    pub fn mine_block(&mut self) -> Result<Block, BlockchainError> {
        if self.pending_transactions.is_empty() {
            return Err(BlockchainError::NoTransactions);
        }

        let last_block = self.chain.last().unwrap();
        let mut new_block = Block {
            index: last_block.index + 1,
            timestamp: Utc::now(),
            transactions: self.pending_transactions.clone(),
            previous_hash: last_block.hash.clone(),
            hash: String::new(),
            nonce: 0,
        };

        self.proof_of_work(&mut new_block);
        self.chain.push(new_block.clone());
        self.pending_transactions.clear();

        println!("⛏️  Block #{} mined with {} transactions", 
                 new_block.index, new_block.transactions.len());
        
        Ok(new_block)
    }

    fn proof_of_work(&self, block: &mut Block) {
        while !self.is_hash_valid(&block.hash) {
            block.nonce += 1;
            block.hash = self.calculate_hash(block);
        }
    }

    fn is_hash_valid(&self, hash: &str) -> bool {
        hash.starts_with(&"0".repeat(self.difficulty as usize))
    }

    fn calculate_hash(&self, block: &Block) -> String {
        let block_data = serde_json::json!({
            "index": block.index,
            "timestamp": block.timestamp.to_rfc3339(),
            "transactions": serde_json::to_value(&block.transactions).unwrap(),
            "previous_hash": block.previous_hash,
            "nonce": block.nonce,
        });

        let mut hasher = Sha256::new();
        hasher.update(block_data.to_string().as_bytes());
        let result = hasher.finalize();
        
        hex::encode(result)
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                println!("❌ Invalid previous hash at block {}", i);
                return false;
            }

            let calculated_hash = self.calculate_hash(current);
            if current.hash != calculated_hash {
                println!("❌ Invalid hash at block {}", i);
                return false;
            }

            if !self.is_hash_valid(&current.hash) {
                println!("❌ Invalid proof of work at block {}", i);
                return false;
            }
        }
        true
    }

    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }

    #[allow(dead_code)]
    pub fn get_last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    #[allow(dead_code)]
    pub fn get_pending_transactions_count(&self) -> usize {
        self.pending_transactions.len()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("No transactions to mine")]
    NoTransactions,
    #[allow(dead_code)]
    #[error("Invalid blockchain")]
    InvalidChain,
}