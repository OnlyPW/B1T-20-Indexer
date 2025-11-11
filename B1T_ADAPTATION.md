# B1T-20 Indexer - B1T Blockchain Anpassung

## Übersicht

Dieser BEL-20-Indexer wurde für die B1T-Blockchain angepasst. Die wichtigsten Änderungen:

## Blockchain-Parameter

### B1T Core Parameter
- **P2PKH Address Prefix**: 25 (Adressen beginnen mit "B")
- **P2SH Address Prefix**: 22
- **Network Magic**: 0x42, 0x31, 0x54, 0x46 ("B1TF")
- **Default Port**: 33317
- **Genesis Block**: 0x456ef94b5dd68b7a96438e3e2c551ddecd7b715cfe011228ac049a2482259862
- **AuxPoW Chain ID**: 0x0B18

### Angepasste Konfiguration

1. **Coin Definition** in `packages/new-blk-parser/src/blockchain/coins.rs`:
   - `B1TCore` mit Address-Prefix 25
   - `B1TCoreTestnet` mit Address-Prefix 65

2. **Blockchain Enum** in `src/blockchain.rs`:
   - Hinzugefügt: `B1TCore` Variant

3. **Start-Parameter** in `src/main.rs`:
   - `START_HEIGHT: 0` (B1T startet von Block 0)
   - `JUBILEE_HEIGHT: 0` (Von Anfang an aktiv)

4. **Server-Konfiguration** in `src/server/mod.rs`:
   - Mapping für "b1tcore" und "b1tcore-testnet"

## Installation

### Voraussetzungen
- B1TCore Node (siehe Installation unten)
- Rust Toolchain
- Build Tools

### B1TCore Node Setup

1. B1TCore herunterladen und installieren:
```bash
mkdir -p B1TCore && cd B1TCore
wget https://github.com/bittoshimoto/Bit/releases/download/Bit.v3/bit.v3.tar.gz
tar -xzf bit.v3.tar.gz
```

2. Abhängigkeiten installieren:
```bash
apt install -y libboost-filesystem1.74-dev libboost-program-options1.74-dev \
libboost-thread1.74-dev libboost-chrono1.74-dev libdb5.3-dev \
libminiupnpc-dev libevent-dev libzmq3-dev libdb++-dev
```

3. Konfiguration anpassen (`~/.bit/bit.conf`):
```
# B1TCore Configuration für Indexing
server=1
daemon=1
listen=1
rpcuser=bitcorerpc
rpcpassword=bitcorerpc123
rpcallowip=127.0.0.1
rpcport=9332
port=9333
maxconnections=125
txindex=1
printtoconsole=1

# Optimierte Einstellungen für Blockchain-Indexing
blockmaxsize=2000000
maxtxsize=100000
rpcthreads=8
rpcworkqueue=128
rpcservertimeout=60
dbcache=512
maxsigcachesize=128
maxorphantx=100
maxmempool=300
mempoolexpiry=72

# Logging für Debugging
debug=rpc
debug=blk
debug=tx
```

### Indexer Setup

1. Repository klonen:
```bash
git clone https://github.com/bittoshimoto/b1t-20-indexer.git
cd b1t-20-indexer
```

2. Umgebungsvariablen setzen (`.env`):
```
RPC_URL=http://localhost:9332
RPC_USER=bitcorerpc
RPC_PASS=bitcorerpc123
BLOCKCHAIN=b1tcore
NETWORK=mainnet
SERVER_BIND_URL=0.0.0.0:8080
BLK_DIR=/root/.bit/blocks
```

3. Bauen und starten:
```bash
cargo build --release
cargo run --release
```

## API-Endpunkte

Der Indexer bietet folgende API-Endpunkte:

- `GET /address/:address` - Token-Balances für Adresse
- `GET /address/:address/history` - Transaktionshistorie
- `GET /events/:height` - Events pro Block-Höhe
- `GET /txid/:txid` - Events nach Transaktions-ID
- `GET /tokens` - Alle Token-Metadaten
- `GET /status` - Status des Servers

## Unterschiede zu BEL-20

1. **Früher Start**: B1T-20 startet bei Block 0, BEL-20 bei Block 26.371
2. **Andere Address-Prefixe**: B1T verwendet 25 (B-Adressen), BEL verwendet 25 (B-Adressen)
3. **Eigene Blockchain**: B1T hat eigene Netzwerkparameter und Genesis-Block

## Testing

Verbindung zum B1TCore testen:
```bash
cd B1TCore
./bit-cli getblockchaininfo
./bit-cli getnetworkinfo
```

Indexer starten und testen:
```bash
curl http://localhost:8080/status
```