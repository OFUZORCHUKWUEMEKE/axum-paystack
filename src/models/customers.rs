// use chrono::Utc;
use mongodb::bson::{DateTime as BsonDateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomerType {
    Regular,
    Premium,
    Vip,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankDetails {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub bank_name: String,
    pub account_number: String,
    pub account_name: String,
    pub customer_id: String,
    #[serde(default = "BsonDateTime::now", rename = "created_at")]
    pub created_at: BsonDateTime,
    #[serde(default = "BsonDateTime::now", rename = "updated_at")]
    pub updated_at: BsonDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(default = "default_true")]
    pub is_active: bool,

    #[serde(default = "default_customer_type")]
    pub role: CustomerType,

    #[serde(default)]
    pub twofa: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_authentication_secret: Option<String>,

    #[serde(default)]
    pub blacklisted: bool,

    #[serde(default = "BsonDateTime::now", rename = "created_at")]
    pub created_at: BsonDateTime,

    #[serde(default = "BsonDateTime::now", rename = "updated_at")]
    pub updated_at: BsonDateTime,
}

fn default_true() -> bool {
    true
}

fn default_customer_type() -> CustomerType {
    CustomerType::Regular
}
