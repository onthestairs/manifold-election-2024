#![recursion_limit = "512"]

use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

use election_2024::{ConstituencyStatus, Party, PartyName, Status};

fn main() {
    let input = std::fs::read("out/constituencies.json").unwrap();
    let constituencies: Status = serde_json::from_slice(&input).unwrap();

    let stats = get_stats(&constituencies.constituencies);

    let monte_carlo_results = run_monte_carlo(&constituencies.constituencies);
    let summaries = get_montecarlo_summary(&monte_carlo_results);

    let mut sorted_stats: Vec<(&PartyName, &i32)> = stats.iter().collect();
    sorted_stats.sort_by(|a, b| (b.1, b.0).cmp(&(a.1, a.0)));

    let html = render_html(&constituencies, &summaries, &sorted_stats);
    // save to file
    std::fs::write("out/index.html", html).unwrap();
}

fn render_html(
    status: &Status,
    summaries: &Vec<MonteCarloSummary>,
    stats: &Vec<(&PartyName, &i32)>,
) -> String {
    let tree = html::root::Html::builder()
        .lang("en")
        .head(|head| {
            head.meta(|meta| meta.charset("utf-8"))
                .meta(|meta| {
                    meta.name("viewport")
                        .content("width=device-width, initial-scale=1")
                })
                .title(|title| title.text("Manifold UK General Election 2024"))
        })
        .body(|body| {
            body.style("margin: 0 auto; padding: 0 20px; max-width: 820px;");

            let mut heading = html::content::Heading1::builder();
            heading.text("Manifold UK General Election 2024");
            body.push(heading.build());

            let mut fetched_at = html::text_content::Paragraph::builder();
            fetched_at.text(format!(
                "Data fetched at {} UTC",
                status.fetched_at.format("%Y-%m-%d %H:%M:%S")
            ));
            body.push(fetched_at.build());

            body.push(html::text_content::ThematicBreak::builder().build());

            let mut summary_heading = html::content::Heading2::builder();
            summary_heading.text("Monte Carlo simulation results");
            body.push(summary_heading.build());
            let mut summary_paragraph = html::text_content::Paragraph::builder();
            summary_paragraph.text(
                "The following table shows the result of a Monte Carlo simulation. A simulated election is run 10,000 times. For each constituency, a party is return based on the implied probabilities of the market. The mode is the most common number of seats won by that party across all the simulations.",
            );
            body.push(summary_paragraph.build());
            let summary_table = make_summary_table(&summaries);
            body.push(summary_table);

            body.push(html::text_content::ThematicBreak::builder().build());

            let mut summary_heading = html::content::Heading2::builder();
            summary_heading.text("Seat favourites");
            body.push(summary_heading.build());

            let stats_table = make_stats_table(stats);
            body.push(stats_table);

            body.push(html::text_content::ThematicBreak::builder().build());

            let constituency_tables = make_constituency_tables(&status.constituencies);
            body.push(constituency_tables);

            return body;
        })
        .build();

    return tree.to_string();
}

fn make_constituency_tables(constituencies: &[ConstituencyStatus]) -> html::text_content::Division {
    let mut division = html::text_content::Division::builder();
    division.style(
        "display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 1rem;",
    );

    let mut sorted_constituencies = constituencies.iter().collect::<Vec<&ConstituencyStatus>>();
    sorted_constituencies.sort_by(|a, b| a.constituency.cmp(&b.constituency));
    for constituency in &sorted_constituencies {
        let table = make_constituency_table(constituency);
        division.push(table);
    }
    return division.build();
}

fn make_constituency_table(constituency: &ConstituencyStatus) -> html::text_content::Division {
    let mut division = html::text_content::Division::builder();

    let mut heading = html::content::Heading2::builder();
    heading.text(constituency.constituency.clone());
    division.push(heading.build());

    let mut table = html::tables::Table::builder();
    let mut reverse_sorted_parties = constituency.parties.iter().collect::<Vec<&Party>>();
    reverse_sorted_parties.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap());
    for party in &reverse_sorted_parties {
        let row = html::tables::TableRow::builder()
            .table_cell(|cell| {
                cell.text(party.name.to_string());
                cell.text(" ");
                cell.text(party.name.to_emoji());
                return cell;
            })
            .table_cell(|cell| {
                cell.text(format!("{:.2}%", party.probability * 100.0));
                return cell;
            })
            .build();
        table.push(row);
    }
    division.push(table.build());

    let mut link = html::inline_text::Anchor::builder();
    link.href(constituency.manifold_url.clone());
    link.target("_blank");
    link.text("See market on Manifold");
    division.push(link.build());

    return division.build();
}

fn make_summary_table(summaries: &Vec<MonteCarloSummary>) -> html::tables::Table {
    let mut table = html::tables::Table::builder();
    table.table_row(|row| {
        row.table_header(|header| {
            header.text("Party");
            return header;
        });
        row.table_header(|header| {
            header.text("Mode seats [5th - 95th percentile]");
            return header;
        });
        return row;
    });

    for summary in summaries.iter() {
        let row = html::tables::TableRow::builder()
            .table_cell(|data| {
                data.text(summary.party.to_string());
                data.text(" ");
                data.text(summary.party.to_emoji());
                return data;
            })
            .table_cell(|data| {
                data.text(summary.mode.to_string());
                data.text(" [");
                data.text(summary.lower_5th.to_string());
                data.text(" - ");
                data.text(summary.upper_95th.to_string());
                data.text("]");
                return data;
            })
            .build();
        table.push(row);
    }
    return table.build();
}

fn make_stats_table(stats: &Vec<(&PartyName, &i32)>) -> html::tables::Table {
    let mut table = html::tables::Table::builder();
    table.table_row(|row| {
        row.table_header(|header| {
            header.text("Party");
            return header;
        });
        row.table_header(|header| {
            header.text("Number of seats favourite to win");
            return header;
        });
        return row;
    });

    for (party, count) in stats.iter() {
        let row = html::tables::TableRow::builder()
            .table_cell(|data| {
                data.text(party.to_string());
                data.text(" ");
                data.text(party.to_emoji());
                return data;
            })
            .table_cell(|data| {
                data.text(count.to_string());
                return data;
            })
            .build();
        table.push(row);
    }
    return table.build();
}

fn run_monte_carlo(constituencies: &Vec<ConstituencyStatus>) -> Vec<HashMap<PartyName, i32>> {
    // run a monte carlo simulation
    // for each constituency, pick a party based on the probabilities
    // and increment the count for that party
    // do this a few thousand times
    // and then output the results
    let mut rng = rand::thread_rng();

    let simulations = 10_000;
    let mut simulation_results: Vec<HashMap<PartyName, i32>> = Vec::new();
    for _ in 0..simulations {
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

struct MonteCarloSummary {
    party: PartyName,
    seats: Vec<i32>,
    mode: i32,
    median: i32,
    lower_5th: i32,
    upper_95th: i32,
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
        let summary = MonteCarloSummary {
            party: party.clone(),
            seats,
            mode,
            median,
            lower_5th,
            upper_95th,
        };
        summaries.push(summary);
    }

    // sort by the mode
    summaries.sort_by(|a, b| b.mode.cmp(&a.mode));

    return summaries;
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
