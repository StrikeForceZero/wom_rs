use crate::models::snapshot::ComputedMetric;
use convert_case::{Case, Casing};
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Metric {
    Skill(Skill),
    Boss(Boss),
    Activity(Activity),
    ComputedMetric(ComputedMetricEnum),
}

impl<'de> Deserialize<'de> for Metric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let metric_str: String = Deserialize::deserialize(deserializer)?;
        let camel_cased = metric_str.to_case(Case::Title).replace(" ", "");

        if let Ok(skill) = Skill::try_from(camel_cased.as_str()) {
            return Ok(Metric::Skill(skill));
        }

        if let Ok(boss) = Boss::try_from(camel_cased.as_str()) {
            return Ok(Metric::Boss(boss));
        }

        if let Ok(activity) = Activity::try_from(camel_cased.as_str()) {
            return Ok(Metric::Activity(activity));
        }

        if let Ok(computed_metric) = ComputedMetricEnum::try_from(camel_cased.as_str()) {
            return Ok(Metric::ComputedMetric(computed_metric));
        }

        Err(serde::de::Error::custom("Unknown metric type"))
    }
}

/// [Skill](https://docs.wiseoldman.net/global-type-definitions#enum-skill)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "camelCase")]
pub enum Skill {
    Overall,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum Boss {
    AbyssalSire,
    AlchemicalHydra,
    Artio,
    BarrowsChests,
    Bryophyta,
    Callisto,
    Calvarion,
    Cerberus,
    ChambersOfXeric,
    ChambersOfXericChallengeMode,
    ChaosElemental,
    ChaosFanatic,
    CommanderZilyana,
    CorporealBeast,
    CrazyArchaeologist,
    DagannothPrime,
    DagannothRex,
    DagannothSupreme,
    DerangedArchaeologist,
    DukeSucellus,
    GeneralGraardor,
    GiantMole,
    GrotesqueGuardians,
    Hespori,
    KalphiteQueen,
    KingBlackDragon,
    Kraken,
    Kreearra,
    KrilTsutsaroth,
    Mimic,
    Nex,
    Nightmare,
    PhosanisNightmare,
    Obor,
    PhantomMuspah,
    Sarachnis,
    Scorpia,
    Scurrius,
    Skotizo,
    Spindel,
    Tempoross,
    TheGauntlet,
    TheCorruptedGauntlet,
    TheLeviathan,
    TheWhisperer,
    TheatreOfBlood,
    TheatreOfBloodHardMode,
    ThermonuclearSmokeDevil,
    TombsOfAmascut,
    TombsOfAmascutExpert,
    TzkalZuk,
    TztokJad,
    Vardorvis,
    Venenatis,
    Vetion,
    Vorkath,
    Wintertodt,
    Zalcano,
    Zulrah,
}

/// [Computed Metric](https://docs.wiseoldman.net/global-type-definitions#enum-computed-metric)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "camelCase")]
pub enum ComputedMetricEnum {
    Ehp,
    Ehb,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
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
