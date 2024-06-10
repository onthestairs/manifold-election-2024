use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

use election_2024::{
    to_simple_summary, AggregatedStats, ConstituencyAggregated, ConstituencyStats,
    ConstituencyStatus, MonteCarloSummary, Party, PartyName, Status, NUMBER_OF_SIMULATIONS,
};

fn main() {
    let input = std::fs::read("out/constituencies.json").unwrap();
    let constituencies: Status = serde_json::from_slice(&input).unwrap();

    let constituencies_aggregated = constituencies
        .constituencies
        .iter()
        .map(|constituency| {
            let stats = make_constituency_stats(&constituency.parties);
            return ConstituencyAggregated {
                constituency: constituency.constituency.clone(),
                parties: constituency.parties.clone(),
                manifold_url: constituency.manifold_url.clone(),
                stats,
            };
        })
        .collect();

    let stats = get_stats(&constituencies.constituencies);

    let monte_carlo_results = run_monte_carlo(&constituencies.constituencies);
    let monte_carlo_summaries = get_montecarlo_summary(&monte_carlo_results);
    let monte_carlo_summaries_simple = monte_carlo_summaries
        .iter()
        .map(to_simple_summary)
        .collect();

    let mut sorted_stats: Vec<(PartyName, i32)> = stats.into_iter().collect();
    sorted_stats.sort_by(|a, b| (b.1, &b.0).cmp(&(a.1, &a.0)));

    let aggregates = AggregatedStats {
        fetched_at: constituencies.fetched_at,
        constituencies: constituencies_aggregated,
        winning_constituencies: sorted_stats,
        monte_carlo_summary: monte_carlo_summaries_simple,
    };

    // output the aggregate stats
    let output = serde_json::to_string(&aggregates).unwrap();
    std::fs::write("out/election-2024.json", output).unwrap();
}

fn make_constituency_stats(parties: &Vec<Party>) -> ConstituencyStats {
    let find_probability = |party_name: PartyName| {
        parties
            .iter()
            .find(|party| party.name == party_name)
            .map(|party| party.probability)
    };
    let labour_probability = find_probability(PartyName::Labour);
    let conservative_probability = find_probability(PartyName::Conservatives);
    let lib_dem_probability = find_probability(PartyName::LiberalDemocrats);
    let green_probability = find_probability(PartyName::Green);
    let reform_probability = find_probability(PartyName::Reform);
    let other_probability = find_probability(PartyName::Other);
    let favourite_percentage = parties.iter().nth(0).map(|party| party.probability);
    let second_favourite_percentage = parties.iter().nth(1).map(|party| party.probability);
    let favourite_lead = favourite_percentage
        .zip(second_favourite_percentage)
        .map(|(favourite, second_favourite)| favourite - second_favourite);
    let third_place_probability = parties.iter().nth(2).map(|party| party.probability);

    return ConstituencyStats {
        labour_probability,
        conservative_probability,
        lib_dem_probability,
        green_probability,
        reform_probability,
        other_probability,
        favourite_lead,
        third_place_probability,
    };
}

fn median(xs: &[i32]) -> i32 {
    return xs[xs.len() / 2];
}

fn mode(xs: &[i32]) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for x in xs {
        if counts.contains_key(x) {
            counts.insert(*x, counts.get(x).unwrap() + 1);
        } else {
            counts.insert(*x, 1);
        }
    }
    let mut max_count = 0;
    let mut mode = 0;
    for (x, count) in counts {
        if count > max_count {
            max_count = count;
            mode = x;
        }
    }
    return mode;
}

fn get_stats(constituencies: &Vec<ConstituencyStatus>) -> HashMap<PartyName, i32> {
    // figure out the most likely party in each constituency

    let mut party_counts: HashMap<PartyName, i32> = HashMap::new();
    for constituency in constituencies {
        let winner = constituency
            .parties
            .iter()
            .max_by(|a, b| a.probability.partial_cmp(&b.probability).unwrap())
            .unwrap();

        if party_counts.contains_key(&winner.name) {
            party_counts.insert(
                winner.name.clone(),
                party_counts.get(&winner.name).unwrap() + 1,
            );
        } else {
            party_counts.insert(winner.name.clone(), 1);
        }
    }
    return party_counts;
}

fn get_montecarlo_summary(
    simulation_results: &Vec<HashMap<PartyName, i32>>,
) -> Vec<MonteCarloSummary> {
    let parties: HashSet<PartyName> = simulation_results
        .iter()
        .flat_map(|party_counts| party_counts.keys())
        .cloned()
        .collect();
    let mut summaries: Vec<MonteCarloSummary> = Vec::new();
    for party in parties {
        let mut seats: Vec<i32> = simulation_results
            .iter()
            .map(|party_counts| *party_counts.get(&party).unwrap_or(&0))
            .collect();
        seats.sort();
        let mode = mode(&seats);
        let lower_5th = seats[(0.05 * seats.len() as f64) as usize];
        let upper_95th = seats[(0.95 * seats.len() as f64) as usize];
        let median = median(&seats);
        let majority_percentage =
            seats.iter().filter(|&x| *x > 325).count() as f64 / seats.len() as f64;
        let summary = MonteCarloSummary {
            party: party.clone(),
            seats,
            mode,
            median,
            lower_5th,
            upper_95th,
            majority_percentage,
        };
        summaries.push(summary);
    }

    // sort by the mode
    summaries.sort_by(|a, b| b.median.cmp(&a.median));

    return summaries;
}

fn run_monte_carlo(constituencies: &Vec<ConstituencyStatus>) -> Vec<HashMap<PartyName, i32>> {
    // run a monte carlo simulation
    // for each constituency, pick a party based on the probabilities
    // and increment the count for that party
    // do this a few thousand times
    // and then output the results
    let mut rng = rand::thread_rng();

    let mut simulation_results: Vec<HashMap<PartyName, i32>> = Vec::new();
    for _ in 0..NUMBER_OF_SIMULATIONS {
        let mut party_counts: HashMap<PartyName, i32> = HashMap::new();
        for constituency in constituencies {
            // randomly pick a party based on the probabilities
            let winner = constituency
                .parties
                .choose_weighted(&mut rng, |party| party.probability)
                .unwrap();

            if party_counts.contains_key(&winner.name) {
                party_counts.insert(
                    winner.name.clone(),
                    party_counts.get(&winner.name).unwrap() + 1,
                );
            } else {
                party_counts.insert(winner.name.clone(), 1);
            }
        }
        simulation_results.push(party_counts);
    }

    return simulation_results;
}
