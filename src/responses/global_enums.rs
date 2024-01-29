use serde::{Deserialize, Serialize};

/// [Period](https://docs.wiseoldman.net/global-type-definitions#enum-period)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Period {
    #[serde(rename = "five_min")]
    FiveMin,
    Day,
    Week,
    Month,
    Year,
}

/// [Metric](https://docs.wiseoldman.net/global-type-definitions#enum-metric)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Metric {
    Skill,
    Boss,
    Activity,
    ComputedMetric,
}

/// [Skill](https://docs.wiseoldman.net/global-type-definitions#enum-skill)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Skill {
    overall,
    Attack,
    Defence,
    Strength,
    Hitpoints,
    Ranged,
    Prayer,
    Magic,
    Cooking,
    Woodcutting,
    Fletching,
    Fishing,
    Firemaking,
    Crafting,
    Smithing,
    Mining,
    Herblore,
    Agility,
    Thieving,
    Slayer,
    Farming,
    Runecrafting,
    Hunter,
    Construction,
}

/// [Boss](https://docs.wiseoldman.net/global-type-definitions#enum-boss)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Boss {
    AbyssalSire,
    AlchemicalHydra,
    Artio,
    BarrowsChests,
    bryophyta,
    callisto,
    calvarion,
    cerberus,
    chambers_of_xeric,
    chambers_of_xeric_challenge_mode,
    chaos_elemental,
    chaos_fanatic,
    commander_zilyana,
    corporeal_beast,
    crazy_archaeologist,
    dagannoth_prime,
    dagannoth_rex,
    dagannoth_supreme,
    deranged_archaeologist,
    duke_sucellus,
    general_graardor,
    giant_mole,
    grotesque_guardians,
    hespori,
    kalphite_queen,
    king_black_dragon,
    kraken,
    kreearra,
    kril_tsutsaroth,
    mimic,
    nex,
    nightmare,
    phosanis_nightmare,
    obor,
    phantom_muspah,
    sarachnis,
    scorpia,
    scurrius,
    skotizo,
    spindel,
    tempoross,
    the_gauntlet,
    the_corrupted_gauntlet,
    the_leviathan,
    the_whisperer,
    theatre_of_blood,
    theatre_of_blood_hard_mode,
    thermonuclear_smoke_devil,
    tombs_of_amascut,
    tombs_of_amascut_expert,
    tzkal_zuk,
    tztok_jad,
    vardorvis,
    venenatis,
    vetion,
    vorkath,
    wintertodt,
    zalcano,
    zulrah,
}

/// [Computed Metric](https://docs.wiseoldman.net/global-type-definitions#enum-computed-metric)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ComputedMetric {
    Ehp,
    Ehb,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Activity {
    LeaguePoints,
    BountyHunterHunter,
    BountyHunterRogue,
    ClueScrollsAll,
    ClueScrollsBeginner,
    ClueScrollsEasy,
    ClueScrollsMedium,
    ClueScrollsHard,
    ClueScrollsElite,
    ClueScrollsMaster,
    LastManStanding,
    PvpArena,
    SoulWarsZeal,
    GuardiansOfTheRift,
}
