// use chrono::Utc;
use mongodb::bson::{DateTime as BsonDateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Pending,
    Success,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Transfer,
    Withdraw,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "_id")]
    id: ObjectId,

    amount: f64,

    #[serde(default = "default_transaction_status")]
    pub status: TransactionStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default = "BsonDateTime::now", rename = "created_at")]
    pub created_at: BsonDateTime,

    #[serde(default = "BsonDateTime::now", rename = "updated_at")]
    pub updated_at: BsonDateTime,
}

fn default_transaction_status() -> TransactionStatus {
    TransactionStatus::Pending
}
