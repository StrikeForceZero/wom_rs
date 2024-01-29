use crate::responses::global_enums::Skill;
use crate::responses::snapshot_responses::{Bosses, Skills};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// [Player Type](https://docs.wiseoldman.net/players-api/player-type-definitions#enum-player-type)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlayerType {
    Unknown,
    Regular,
    Ironman,
    Hardcore,
    Ultimate,
}

/// [Player Build](https://docs.wiseoldman.net/players-api/player-type-definitions#enum-player-build)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlayerBuild {
    Main,
    F2p,
    Lvl3,
    Zerker,
    Def1,
    Hp10,
    F2pLvl3,
}

/// [Player Status](https://docs.wiseoldman.net/players-api/player-type-definitions#enum-player-status)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlayerStatus {
    Active,
    Unranked,
    Flagged,
    Archived,
    Banned,
}

/// [Country](https://docs.wiseoldman.net/players-api/player-type-definitions#enum-country)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Country {
    AD,
    AE,
    AF,
    AG,
    AI,
    AL,
    AM,
    AO,
    AQ,
    AR,
    AS,
    AT,
    AU,
    AW,
    AX,
    AZ,
    BA,
    BB,
    BD,
    BE,
    BF,
    BG,
    BH,
    BI,
    BJ,
    BL,
    BM,
    BN,
    BO,
    BQ,
    BR,
    BS,
    BT,
    BV,
    BW,
    BY,
    BZ,
    CA,
    CC,
    CD,
    CF,
    CG,
    CH,
    CI,
    CK,
    CL,
    CM,
    CN,
    CO,
    CR,
    CU,
    CV,
    CW,
    CX,
    CY,
    CZ,
    DE,
    DJ,
    DK,
    DM,
    DO,
    DZ,
    EC,
    EE,
    EG,
    EH,
    ER,
    ES,
    ET,
    FI,
    FJ,
    FK,
    FM,
    FO,
    FR,
    GA,
    GB,
    GD,
    GE,
    GF,
    GG,
    GH,
    GI,
    GL,
    GM,
    GN,
    GP,
    GQ,
    GR,
    GS,
    GT,
    GU,
    GW,
    GY,
    HK,
    HM,
    HN,
    HR,
    HT,
    HU,
    ID,
    IE,
    IL,
    IM,
    IN,
    IO,
    IQ,
    IR,
    IS,
    IT,
    JE,
    JM,
    JO,
    JP,
    KE,
    KG,
    KH,
    KI,
    KM,
    KN,
    KP,
    KR,
    KW,
    KY,
    KZ,
    LA,
    LB,
    LC,
    LI,
    LK,
    LR,
    LS,
    LT,
    LU,
    LV,
    LY,
    MA,
    MC,
    MD,
    ME,
    MF,
    MG,
    MH,
    MK,
    ML,
    MM,
    MN,
    MO,
    MP,
    MQ,
    MR,
    MS,
    MT,
    MU,
    MV,
    MW,
    MX,
    MY,
    MZ,
    NA,
    NC,
    NE,
    NF,
    NG,
    NI,
    NL,
    NO,
    NP,
    NR,
    NU,
    NZ,
    OM,
    PA,
    PE,
    PF,
    PG,
    PH,
    PK,
    PL,
    PM,
    PN,
    PR,
    PS,
    PT,
    PW,
    PY,
    QA,
    RE,
    RO,
    RS,
    RU,
    RW,
    SA,
    SB,
    SC,
    SD,
    SE,
    SG,
    SH,
    SI,
    SJ,
    SK,
    SL,
    SM,
    SN,
    SO,
    SR,
    SS,
    ST,
    SV,
    SX,
    SY,
    SZ,
    TC,
    TD,
    TF,
    TG,
    TH,
    TJ,
    TK,
    TL,
    TM,
    TN,
    TO,
    TR,
    TT,
    TV,
    TW,
    TZ,
    UA,
    UG,
    UM,
    US,
    UY,
    UZ,
    VA,
    VC,
    VE,
    VG,
    VI,
    VN,
    VU,
    WF,
    WS,
    YE,
    YT,
    ZA,
    ZM,
    ZW,
}

/// [Achievement Measure](https://docs.wiseoldman.net/players-api/player-type-definitions#enum-achievement-measure)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AchievementMeasure {
    Levels,
    Experience,
    Kills,
    Score,
    Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapShotData {
    pub skills: Skills,
    pub bosses: Bosses,
    // pub activities: Activities,
    // pub computed: Computed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapShot {
    id: i64,
    player_id: i64,
    created_at: DateTime<Utc>,
    imported_at: Option<DateTime<Utc>>,
    data: SnapShotData,
}

/// [Achievement](https://docs.wiseoldman.net/players-api/player-type-definitions#object-achievement)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    player_id: i64,
    name: String,
    metric: String,
    measure: AchievementMeasure,
    threshold: i64,
    accuracy: Option<i64>,
}

/// [Player](https://docs.wiseoldman.net/players-api/player-type-definitions#object-player)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub player_type: PlayerType,
    pub build: PlayerBuild,
    pub country: Option<Country>,
    pub status: PlayerStatus,
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

/// [Player Details](https://docs.wiseoldman.net/players-api/player-type-definitions#object-player-details)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDetails {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub player_type: PlayerType,
    pub build: PlayerBuild,
    pub country: Option<Country>,
    pub status: PlayerStatus,
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
    pub combat_level: i64,
    pub archive: PlayerArchive,
    pub latest_snapshot: Option<SnapShot>,
}

/// [Player Archive](https://docs.wiseoldman.net/players-api/player-type-definitions#object-player-archive)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerArchive {
    pub player_id: i64,
    pub previous_username: String,
    pub archive_username: String,
    pub restored_username: Option<String>,
    pub created_at: DateTime<Utc>,
    pub restored_at: Option<DateTime<Utc>>,
}
