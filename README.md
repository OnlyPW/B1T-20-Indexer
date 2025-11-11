# B1T-20 Token Indexer

A high-performance token indexer for the B1T blockchain, implementing the B1T-20 token standard (inspired by BRC-20).

## Features

- **Fast Indexing**: Direct blockchain file parsing for 5-20x faster indexing
- **B1T-20 Support**: Full implementation of the B1T-20 token standard
- **REST API**: Complete API for token queries, address balances, and transaction history
- **Real-time Events**: WebSocket support for live token events
- **Proof of History**: Built-in blockchain verification

## Quick Start

### Prerequisites

- Rust 1.70+ and Cargo
- Running B1TCore node with RPC enabled
- B1T blockchain data (blk*.dat files)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd b1t-20-indexer
```

2. Configure environment:
```bash
cp .env.example .env
# Edit .env with your B1TCore node details
```

3. Build and run:
```bash
cargo build --release
./target/release/bel_20_node
```

### Configuration (.env)

```bash
# RPC connection to B1TCore node
RPC_URL=http://localhost:9332
RPC_USER=your_rpc_user
RPC_PASS=your_rpc_password

# Blockchain configuration
BLOCKCHAIN=b1tcore
NETWORK=mainnet

# Optional: Path to B1T blockchain blk*.dat files (faster indexing)
BLK_DIR=/home/user/.bit/blocks

# Server binding
SERVER_BIND_URL=0.0.0.0:8080
```

## API Endpoints

### Address Management
- `GET /address/:address` - Token balances for an address
- `GET /address/:address/history` - Transaction history
- `GET /address/:address/tokens` - Token holdings
- `GET /address/:address/:tick/balance` - Specific token balance

### Token Management
- `GET /tokens` - List all tokens
- `GET /token?tick=:tick` - Specific token info
- `GET /token-events/:tick` - Token events
- `GET /holders` - Token holders
- `GET /holders-stats` - Holder statistics

### Transaction & Events
- `GET /txid/:txid` - Transactions by ID
- `GET /events/:height` - Events by block height
- `POST /events` - Subscribe to events (WebSocket)

### Status & Verification
- `GET /status` - Server status
- `GET /proof-of-history` - Blockchain proof
- `GET /all-addresses` - All indexed addresses
- `GET /all-tickers` - All token tickers

### API Documentation
- `GET /docs` - Interactive API documentation

## Performance Optimization

### Fast Indexing with BLK Files

For optimal performance, configure direct access to blockchain files:

1. Ensure your B1TCore node is stopped:
```bash
# If using systemd
sudo systemctl stop b1tcore
```

2. Set `BLK_DIR` in `.env`:
```bash
BLK_DIR=/home/user/.bit/blocks
```

3. Restart the indexer - it will now parse blocks directly from disk

### Database

The indexer uses RocksDB for storage (default: `./rocksdb/`). To change location:
```bash
DB_PATH=/path/to/custom/db
```

## Development

### Project Structure

```
b1t-20-indexer/
├── src/
│   ├── main.rs              # Entry point
│   ├── rest/                # API endpoints
│   ├── inscriptions/        # B1T-20 parser
│   ├── tokens/              # Token logic
│   └── blockchain.rs        # Blockchain interface
├── packages/
│   ├── new-blk-parser/      # Blockchain parser
│   └── rocksdb-wrapper/     # Database layer
└── Cargo.toml
```

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

## B1T-20 Token Standard

B1T-20 is a token standard for the B1T blockchain, inspired by BRC-20. It enables:

- **Deploy**: Create new tokens
- **Mint**: Generate token supply
- **Transfer**: Move tokens between addresses

Tokens are created through inscriptions in the B1T blockchain.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
