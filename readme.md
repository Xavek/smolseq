# Smol MEV-Resistant Sequencer Demo

A simple demonstration of how transaction ordering affects MEV (Maximum Extractable Value) in blockchain sequencers.

## What is MEV?
MEV is profit extracted by reordering, inserting, or censoring transactions. Traditional sequencers that order by gas price are vulnerable to:
- **Frontrunning**: Copying profitable transactions with higher gas
- **Sandwiching**: Placing transactions before/after targets to manipulate prices

## How This Demo Works

### Three Ordering Strategies:
1. **Vulnerable (Gas Price)**: Orders by highest gas price → MEV exploitable
2. **Fair (Timestamp)**: Orders by arrival time → MEV resistant  
3. **Randomized**: Random order within batches → MEV resistant

### Run the Demo:
```bash
cargo run
