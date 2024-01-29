use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// https://docs.wiseoldman.net/players-api/player-type-definitions#object-player
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub player_type: String,
    pub build: String,
    pub country: Option<String>,
    pub status: String,
    pub patron: bool,
    pub exp: i64,
    pub ehp: f64,
    pub ehb: f64,
    pub ttm: f64,
    pub tt200m: f64,
    pub registered_at: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_changed_at: Option<DateTime<Utc>>,
    pub last_imported_at: Option<DateTime<Utc>>,
}
