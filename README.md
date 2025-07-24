# blockchain-light-node

A Rust‑based light client that syncs block headers from Moca Chain, verifies proof-of-work/state, and exposes a JSON‑RPC interface—ideal for bandwidth‑ and storage‑constrained environments.

### Features
#### Header Synchronization
- Connects to full nodes via JSON‑RPC or P2P peers
- Downloads only block headers (not full blocks)
- Efficient checkpointing to resume sync after restarts

#### Proof Verification
- Validates Proof‑of‑Work (Bitcoin) or block seals (Ethereum)
- Verifies Merkle‑Mountain-Range (MMR) or skip‑list proofs for historical headers
- Ensures chain validity and prevents spoofed headers

####  Light Client API
= Implements a minimal JSON‑RPC server (methods like eth_syncing, eth_getBlockByNumber, btc_getbestblockhash)
= Serves header, block‑height, and chain‑status queries
= Pluggable backends for different networks

### Installation
```sh
git clone https://github.com/your-username/blockchain-light-node.git
cd blockchain-light-node
# Ensure Rust toolchain (stable) is installed
cargo build --release
```

### License
This project is released under the MIT License. Contributions are welcome!
