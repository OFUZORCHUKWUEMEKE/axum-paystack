use mongodb::bson::{DateTime as BsonDateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    #[serde(rename = "user_id")]
    user_id: ObjectId,

    #[serde(default = "default_balance")]
    pub balance: f64,

    #[serde(default = "default_currency")]
    currency: String,

    #[serde(default = "BsonDateTime::now", rename = "created_at")]
    pub created_at: BsonDateTime,

    #[serde(default = "BsonDateTime::now", rename = "updated_at")]
    pub updated_at: BsonDateTime,
}

fn default_balance() -> f64 {
    0.0
}

fn default_currency() -> String {
    "NGN".to_string()
}
