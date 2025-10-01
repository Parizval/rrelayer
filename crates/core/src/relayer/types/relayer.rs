use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{RelayerId, WalletIndex};
use crate::{gas::GasPrice, network::ChainId, shared::common_types::EvmAddress};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Relayer {
    /// The unique identifier for the relayer
    pub id: RelayerId,

    /// The name of the relayer
    pub name: String,

    /// The chain id the relayer is operating on
    #[serde(rename = "chainId")]
    pub chain_id: ChainId,

    /// The relayer address
    pub address: EvmAddress,

    /// The relayer wallet index (i32 to support negative indexes for private keys)
    #[serde(rename = "walletIndex")]
    pub wallet_index: i32,

    /// The max gas price
    #[serde(rename = "maxGasPrice", skip_serializing_if = "Option::is_none", default)]
    pub max_gas_price: Option<GasPrice>,

    /// If the relayer is paused
    pub paused: bool,

    /// If 1559 transactions are enabled
    #[serde(rename = "eip1559Enabled")]
    pub eip_1559_enabled: bool,

    /// The relayer creation time
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    /// Whether this relayer uses a private key (vs mnemonic-derived)
    #[serde(rename = "isPrivateKey")]
    pub is_private_key: bool,
}

impl Relayer {
    /// Get the WalletIndex enum for this relayer
    pub fn wallet_index_type(&self) -> WalletIndex {
        if self.is_private_key {
            WalletIndex::PrivateKey(self.wallet_index)
        } else {
            WalletIndex::Normal(self.wallet_index as u32)
        }
    }
}
