use election_2024::{ConstituencyStatus, Party, PartyName};
use serde::Deserialize;

fn main() {
    // the group which contains all the markets
    let group_id = "f763184a-51f4-4de2-a9df-d290134e6298";
    let markets = get_all_markets_in_group(&group_id);

    let mut constituencies: Vec<ConstituencyStatus> = Vec::new();

    for market in markets.iter().take(5000) {
        let market_detailed = get_market_answer_probabiities(&market.id);
        let constituency_name = extract_constituency_name(&market.question);

        let mut parties: Vec<Party> = Vec::new();
        for answer in &market_detailed.answers {
            let party = Party {
                name: parse_party_name(&answer.text),
                probability: answer.probability,
            };
            parties.push(party);
        }

        let constituency = ConstituencyStatus {
            constituency: constituency_name,
            parties: parties,
        };

        constituencies.push(constituency);
    }

    // output the stats
    let output = serde_json::to_string(&constituencies).unwrap();
    std::fs::write("out/constituencies.json", output).unwrap();
}

fn parse_party_name(party_name: &str) -> PartyName {
    let trimmed_name = party_name.trim();
    return match trimmed_name {
        "Conservative" => PartyName::Conservatives,
        "Liberal Democrat" => PartyName::LiberalDemocrats,
        "Lib Dem" => PartyName::LiberalDemocrats,
        "Labour" => PartyName::Labour,
        "Conservatives" => PartyName::Conservatives,
        "Liberal Democrats" => PartyName::LiberalDemocrats,
        "Scottish National Party" => PartyName::SNP,
        "Sinn Féin" => PartyName::SinnFein,
        "Democratic Unionist Party" => PartyName::DUP,
        "Plaid Cymru" => PartyName::PlaidCymru,
        "Alliance" => PartyName::Alliance,
        "Social Democratic and Labour Party" => PartyName::SDLP,
        "Green" => PartyName::Green,
        "Independent: Jeremy Corbyn" => PartyName::Independent,
        "Independent: Sir Lindsay Hoyle" => PartyName::Independent,
        "Independent: Andrew Bridgen" => PartyName::Independent,
        "Workers Party of Britain" => PartyName::WorkersPartyOfBritain,
        "Other" => PartyName::Other,
        _ => PartyName::Unparsed(trimmed_name.to_string()),
    };
}

fn extract_constituency_name(market_question: &str) -> String {
    // question will be of the form `UK General Election: Which party will win in Altrincham and Sale West?`
    // we want to extract `Altrincham and Sale West`

    let mut parts: Vec<&str> = market_question.split("Which party will win in ").collect();
    if parts.len() < 2 {
        // some markets omit the 'in'
        parts = market_question.split("Which party will win ").collect();
    }
    let constituency_name = parts[1].split("?").collect::<Vec<&str>>()[0];
    return constituency_name.to_string();
}

#[derive(Debug, Deserialize, Clone)]
struct Market {
    id: String,
    question: String,
}

fn get_all_markets_in_group(group_id: &str) -> Vec<Market> {
    // include 1000 markets per page
    let url = format!(
        "https://api.manifold.markets/v0/markets?groupId={}&limit=1000",
        group_id
    );
    let response = reqwest::blocking::get(&url).unwrap();
    let markets: Vec<Market> = response.json().unwrap();
    return markets;
}

#[derive(Debug, Deserialize)]
struct MarketDetailed {
    answers: Vec<MarketAnswer>,
}

#[derive(Debug, Deserialize)]
struct MarketAnswer {
    text: String,
    probability: f64,
}

fn get_market_answer_probabiities(market_id: &str) -> MarketDetailed {
    let url = format!("https://api.manifold.markets/v0/market/{}", market_id);
    let response = reqwest::blocking::get(&url).unwrap();
    let market_detailed: MarketDetailed = response.json().unwrap();
    return market_detailed;
}
