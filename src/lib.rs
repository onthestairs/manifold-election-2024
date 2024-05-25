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

#[derive(Eq, Serialize, Deserialize, PartialEq, Hash, Clone, Debug)]
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
            PartyName::Other => "Other".to_string(),
            PartyName::Unparsed(name) => name.to_string(),
        }
    }
}
