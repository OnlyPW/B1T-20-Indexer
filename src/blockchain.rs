use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blockchain {
    Dogecoin,
    Bellscoin,
    Pepecoin,
    Litecoin,
    B1TCore,
}

#[derive(Debug, thiserror::Error)]
pub enum BlockchainParseError {
    #[error("Unknown blockchain")]
    UnknownBlockchain,
}

impl FromStr for Blockchain {
    type Err = BlockchainParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "dogecoin" | "doge" => Ok(Blockchain::Dogecoin),
            "bellscoin" | "bells" => Ok(Blockchain::Bellscoin),
            "pepecoin" | "pepe" => Ok(Blockchain::Pepecoin),
            "litecoin" => Ok(Blockchain::Litecoin),
            "b1tcore" | "b1t" => Ok(Blockchain::B1TCore),
            _ => Err(BlockchainParseError::UnknownBlockchain),
        }
    }
}
