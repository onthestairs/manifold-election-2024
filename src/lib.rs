use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Status {
    pub fetched_at: DateTime<Utc>,
    pub constituencies: Vec<ConstituencyStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstituencyStatus {
    pub constituency: String,
    pub parties: Vec<Party>,
    pub manifold_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Party {
    pub name: PartyName,
    pub probability: f64,
}

#[derive(Eq, Ord, PartialOrd, Serialize, Deserialize, PartialEq, Hash, Clone, Debug)]
pub enum PartyName {
    Conservatives,
    Labour,
    LiberalDemocrats,
    SNP,
    Green,
    PlaidCymru,
    DUP,
    SinnFein,
    SDLP,
    Alliance,
    Independent,
    WorkersPartyOfBritain,
    Reform,
    Other,
    Unparsed(String),
}

impl PartyName {
    pub fn to_string(&self) -> String {
        match self {
            PartyName::Conservatives => "Conservatives".to_string(),
            PartyName::Labour => "Labour".to_string(),
            PartyName::LiberalDemocrats => "Liberal Democrats".to_string(),
            PartyName::SNP => "Scottish National Party".to_string(),
            PartyName::Green => "Green".to_string(),
            PartyName::PlaidCymru => "Plaid Cymru".to_string(),
            PartyName::DUP => "Democratic Unionist Party".to_string(),
            PartyName::SinnFein => "Sinn Féin".to_string(),
            PartyName::SDLP => "Social Democratic and Labour Party".to_string(),
            PartyName::Alliance => "Alliance".to_string(),
            PartyName::Independent => "Independent".to_string(),
            PartyName::WorkersPartyOfBritain => "Workers Party of Britain".to_string(),
            PartyName::Reform => "Reform".to_string(),
            PartyName::Other => "Other".to_string(),
            PartyName::Unparsed(name) => name.to_string(),
        }
    }
}

impl PartyName {
    pub fn to_emoji(&self) -> String {
        match self {
            PartyName::Conservatives => "🌳".to_string(),
            PartyName::Labour => "🌹".to_string(),
            PartyName::LiberalDemocrats => "🕊️".to_string(),
            PartyName::SNP => "🎗️".to_string(),
            PartyName::Green => "🌱".to_string(),
            PartyName::PlaidCymru => "🌼".to_string(),
            PartyName::DUP => "🦁".to_string(),
            PartyName::SinnFein => "🇮🇪".to_string(),
            PartyName::SDLP => "".to_string(),
            PartyName::Alliance => "".to_string(),
            PartyName::Independent => "".to_string(),
            PartyName::WorkersPartyOfBritain => "⚙️".to_string(),
            PartyName::Reform => "".to_string(),
            PartyName::Other => "".to_string(),
            PartyName::Unparsed(_) => "".to_string(),
        }
    }
}

///////// Aggregate Stats

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggregatedStats {
    pub fetched_at: DateTime<Utc>,
    pub constituencies: Vec<ConstituencyAggregated>,
    pub winning_constituencies: Vec<(PartyName, i32)>,
    pub monte_carlo_summary: Vec<MonteCarloSummarySimple>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstituencyStats {
    pub labour_probability: Option<f64>,
    pub conservative_probability: Option<f64>,
    pub lib_dem_probability: Option<f64>,
    pub green_probability: Option<f64>,
    pub reform_probability: Option<f64>,
    pub other_probability: Option<f64>,
    pub favourite_lead: Option<f64>,
    pub third_place_probability: Option<f64>,
}

pub const NUMBER_OF_SIMULATIONS: usize = 100_000;

pub struct MonteCarloSummary {
    pub party: PartyName,
    pub seats: Vec<i32>,
    pub mode: i32,
    pub median: i32,
    pub lower_5th: i32,
    pub upper_95th: i32,
    pub majority_percentage: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonteCarloSummarySimple {
    pub party: PartyName,
    pub mode: i32,
    pub median: i32,
    pub lower_5th: i32,
    pub upper_95th: i32,
    pub majority_percentage: f64,
}

pub fn to_simple_summary(summary: &MonteCarloSummary) -> MonteCarloSummarySimple {
    return MonteCarloSummarySimple {
        party: summary.party.clone(),
        mode: summary.mode,
        median: summary.median,
        lower_5th: summary.lower_5th,
        upper_95th: summary.upper_95th,
        majority_percentage: summary.majority_percentage,
    };
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstituencyAggregated {
    pub constituency: String,
    pub parties: Vec<Party>,
    pub manifold_url: String,
    pub stats: ConstituencyStats,
}
