use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

#[derive(Debug, Deserialize)]
pub struct ApiStatus {
    pub user_count: u64,
    pub server_count: u64,
    pub suspended_server_count: u64,
    pub node_count: u64,
}

#[derive(Debug, Deserialize)]
pub struct ApiState {
    pub maintenance_mode: bool,
    pub available_free_servers: u32,
}

#[derive(Debug, Serialize)]
pub struct PatchStateBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_free_servers: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct OrionUser {
    pub id: String,
    pub panel_id: u64,
    pub discord_id: String,
    pub last_login_at: i64,
    pub username: String,
    pub credits: i64,
    pub referral_code: String,
    pub referral_usage: u64,
    pub referral_gains: i64,
}

#[derive(Debug, Serialize)]
pub struct CreateCreditTransactionBody {
    #[serde(rename = "type")]
    pub kind: CreditTransactionType,
    pub amount: i64,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreditTransactionResult {
    pub transaction_id: String,
    pub user_credits: i64,
}

#[derive(Debug, Serialize_repr, TryFromPrimitive)]
#[repr(u8)]
pub enum CreditTransactionType {
    Custom = 0,
    Giveaway = 1,
    ReferralCode = 2,
    SupportActivity = 3,
    Partnership = 4,
    DiscordSponsoredAd = 5,
    PromotionCode = 6,
    DiscordBoost = 7,
}
