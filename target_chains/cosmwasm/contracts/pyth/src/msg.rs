use {crate::state::PythDataSource, cosmwasm_schema::cw_serde, cosmwasm_std::Coin};

// cw_serde attribute is equivalent to
// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
// #[serde(rename_all = "snake_case")]

type HumanAddr = String;

#[cw_serde]
pub struct InstantiateMsg {
    pub wormhole_contract: HumanAddr,
    pub data_sources: Vec<PythDataSource>,

    pub governance_source: PythDataSource,
    pub governance_source_index: u32,
    pub governance_sequence_number: u64,

    pub chain_id: u16,
    pub valid_time_period_secs: u16,

    pub fee: Coin,
}

/// TEST MODE ONLY: Test price data for direct price injection
#[derive(Eq)]
#[cw_serde]
pub struct TestPrice {
    pub price_id: String,  // hex string (64 chars)
    pub price: i64,
    pub conf: u64,
    pub expo: i32,
    pub ema_price: i64,
    pub ema_conf: u64,
}

#[derive(Eq)]
#[cw_serde]
pub struct MigrateMsg {
    /// TEST MODE ONLY: Set a test price directly without VAA verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_price: Option<TestPrice>,
}
