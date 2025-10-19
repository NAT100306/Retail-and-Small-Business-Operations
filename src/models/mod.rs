use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
    pub manufacturer: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
    pub currency: Currency,
    pub timestamp: DateTime<Utc>,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainRecord {
    pub product_id: Uuid,
    pub location: String,
    pub handler: String,
    pub timestamp: DateTime<Utc>,
    pub action: SupplyChainAction,
    pub metadata: serde_json::Value,
}

// Sửa lại Currency - loại bỏ f64 khỏi các derive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    BTC,
    ETH,
    USDT,
    RETAIL(RetailToken),
}

// Manual implementation của Eq, PartialEq, Hash cho Currency
impl PartialEq for Currency {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Currency::BTC, Currency::BTC) => true,
            (Currency::ETH, Currency::ETH) => true,
            (Currency::USDT, Currency::USDT) => true,
            (Currency::RETAIL(a), Currency::RETAIL(b)) => a.symbol == b.symbol,
            _ => false,
        }
    }
}

impl Eq for Currency {}

impl std::hash::Hash for Currency {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Currency::BTC => "BTC".hash(state),
            Currency::ETH => "ETH".hash(state),
            Currency::USDT => "USDT".hash(state),
            Currency::RETAIL(token) => {
                "RETAIL".hash(state);
                token.symbol.hash(state);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetailToken {
    pub symbol: String,
    pub amount: f64,
    pub loyalty_points: u32,
}

// Manual implementation cho RetailToken
impl PartialEq for RetailToken {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl Eq for RetailToken {}

impl std::hash::Hash for RetailToken {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum SupplyChainAction {
    Manufactured,
    Shipped,
    Received,
    Sold,
}