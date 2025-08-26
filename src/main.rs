use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{rng, seq::SliceRandom};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    id: String,
    rollup: String,
    timestamp: u64,
    gas_price: u64,
    data: String,
}

#[derive(Debug)]
struct SimpleSequencer {
    transactions: Vec<Transaction>,
}

impl SimpleSequencer {
    fn new() -> Self {
        Self { transactions: Vec::new() }
    }
    
    fn add_transaction(&mut self, tx: Transaction) {
        self.transactions.push(tx);
    }
    
    // MEV-vulnerable ordering (by gas price)
    fn create_vulnerable_batch(&self) -> Vec<Transaction> {
        let mut batch = self.transactions.clone();
        batch.sort_by(|a, b| b.gas_price.cmp(&a.gas_price)); // Highest gas first
        batch
    }
    
    fn create_fair_batch(&self) -> Vec<Transaction> {
        let mut batch = self.transactions.clone();
        batch.sort_by_key(|tx| tx.timestamp); // First come, first served
        batch
    }
    
    // MEV-resistant randomized batch
    fn create_random_batch(&self) -> Vec<Transaction> {
        let mut batch = self.transactions.clone();
        batch.shuffle(&mut rng());
        batch
    }
}

fn main() {
    println!("MEV-Resistant Sequencer Demo\n");
    
    let mut sequencer = SimpleSequencer::new();
    
    let rollups = vec!["optimism", "arbitrum", "polygon"];
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    
    // Add some transactions with different gas prices and timestamps
    for i in 0..10 {
        let tx = Transaction {
            id: format!("tx_{}", i),
            rollup: rollups[i % rollups.len()].to_string(),
            timestamp: current_time + (i as u64 * 100), // Spread over time
            gas_price: if i % 3 == 0 { 100 } else { 20 }, // Some high gas, some low
            data: format!("Transfer {} ETH", i + 1),
        };
        sequencer.add_transaction(tx);
    }
    
    // Show the three different ordering strategies
    println!("Transaction Ordering Comparison:\n");
    
    println!("Normal (Gas Price Ordering: MEV Exploitable)");
    for (i, tx) in sequencer.create_vulnerable_batch().iter().enumerate() {
        println!("  {}. {} | {} | Gas: {} | {}", 
            i+1, tx.id, tx.rollup, tx.gas_price, tx.data);
    }
    println!("---------------------------------------------------");
    println!("\n Fair (Timestamp Ordering: MEV Resistant)");
    for (i, tx) in sequencer.create_fair_batch().iter().enumerate() {
        println!("  {}. {} | {} | Time: {} | {}", 
            i+1, tx.id, tx.rollup, tx.timestamp, tx.data);
    }
    println!("----------------------------------------------------");
    println!("\n Random (Random Ordering: MEV Resistant)");
    for (i, tx) in sequencer.create_random_batch().iter().enumerate() {
        println!("  {}. {} | {} | {}", 
            i+1, tx.id, tx.rollup, tx.data);
    }
    println!("----------------------------------------------------");
    let rollup_counts: HashMap<String, usize> = sequencer.transactions
        .iter()
        .fold(HashMap::new(), |mut acc, tx| {
            *acc.entry(tx.rollup.clone()).or_insert(0) += 1;
            acc
        });
    
    println!("\n Sequencer Metrics:");
    println!("Total Transactions: {}", sequencer.transactions.len());
    println!("Rollup Distribution: {:?}", rollup_counts);
    
    println!("\n EOF");
}
