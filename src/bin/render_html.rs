#![recursion_limit = "512"]

use std::collections::HashMap;

use election_2024::{ConstituencyStatus, Party, PartyName};

fn main() {
    let input = std::fs::read("out/constituencies.json").unwrap();
    let constituencies = serde_json::from_slice(&input).unwrap();

    let stats = get_stats(&constituencies);

    let mut sorted_stats: Vec<(&PartyName, &i32)> = stats.iter().collect();
    sorted_stats.sort_by(|a, b| b.1.cmp(a.1));

    for (party, count) in &sorted_stats {
        println!("{:?}: {}", party, count);
    }

    let html = render_html(&constituencies, &sorted_stats);
    // save to file
    std::fs::write("out/index.html", html).unwrap();
}

fn render_html(
    constituencies: &Vec<ConstituencyStatus>,
    stats: &Vec<(&PartyName, &i32)>,
) -> String {
    let tree = html::root::Html::builder()
        .lang("en")
        .head(|head| {
            head.meta(|meta| meta.charset("utf-8"))
                .title(|title| title.text("Manifold UK General Election 2024"))
        })
        .body(|body| {
            let mut heading = html::content::Heading1::builder();
            heading.text("Manifold UK General Election 2024");
            body.push(heading.build());

            let stats_table = make_stats_table(stats);
            body.push(stats_table);

            return body;
        })
        .build();

    return tree.to_string();
}

fn make_stats_table(stats: &Vec<(&PartyName, &i32)>) -> html::tables::Table {
    let mut table = html::tables::Table::builder();
    table.table_row(|row| {
        row.table_header(|header| {
            header.text("Party");
            return header;
        });
        row.table_header(|header| {
            header.text("Projected seats");
            return header;
        });
        return row;
    });

    for (party, count) in stats.iter() {
        let row = html::tables::TableRow::builder()
            .table_cell(|data| {
                data.text(party.to_string());
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
