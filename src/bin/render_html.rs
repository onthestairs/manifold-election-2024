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

            body.push(html::text_content::ThematicBreak::builder().build());

            let constituency_tables = make_constituency_tables(constituencies);
            body.push(constituency_tables);

            return body;
        })
        .build();

    return tree.to_string();
}

fn make_constituency_tables(constituencies: &[ConstituencyStatus]) -> html::text_content::Division {
    let mut division = html::text_content::Division::builder();
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
    return division.build();
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
