#![recursion_limit = "512"]

use election_2024::{
    AggregatedStats, ConstituencyAggregated, MonteCarloSummarySimple, Party, PartyName,
    NUMBER_OF_SIMULATIONS,
};

fn main() {
    let input = std::fs::read("out/election-2024.json").unwrap();
    let constituencies: AggregatedStats = serde_json::from_slice(&input).unwrap();

    let html = render_html(&constituencies);
    // save to file
    std::fs::write("out/index.html", html).unwrap();
    std::fs::copy("src/sort.js", "out/sort.js").unwrap();
}

fn render_html(constituencies: &AggregatedStats) -> String {
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

            let mut intro_paragraph = html::text_content::Paragraph::builder();
            intro_paragraph.text("This page is a dashboard of data concerning the 2024 UK General Election based on data from the prediction market site ");
            intro_paragraph.push(html::inline_text::Anchor::builder().href("https://manifold.markets/home").text("Manifold").build());
            intro_paragraph.text(". It shows the current probability of each party in each constituency and various aggregations.");
            body.push(intro_paragraph.build());

            let mut fetched_at = html::text_content::Paragraph::builder();
            fetched_at.text(format!(
                "Data fetched at {} UTC",
                constituencies.fetched_at.format("%Y-%m-%d %H:%M:%S")
            ));
            body.push(fetched_at.build());

            let mut github_link = html::text_content::Paragraph::builder();
            github_link.text(format!(
                "Open source at "
            ));
            github_link.push(html::inline_text::Anchor::builder().href("https://github.com/onthestairs/manifold-election-2024").text("Github").build());
            github_link.text(".");
            body.push(github_link.build());

            body.push(html::text_content::ThematicBreak::builder().build());

            let mut summary_heading = html::content::Heading2::builder();
            summary_heading.text("Monte Carlo simulation results");
            body.push(summary_heading.build());
            let mut summary_paragraph = html::text_content::Paragraph::builder();
            summary_paragraph.text(
                format!("The following table shows the result of a Monte Carlo simulation. A simulated election is run {} times. For each constituency, a party is returned randomly based on the implied probabilities of the market. The median is the middle number of seats won by that party across all the simulations. The majority percent shows how many times in the simulation the given party wins a majority (>325 seats). ", NUMBER_OF_SIMULATIONS),
            );
            body.push(summary_paragraph.build());
            let summary_table = make_summary_table(&constituencies.monte_carlo_summary);
            body.push(summary_table);

            body.push(html::text_content::ThematicBreak::builder().build());

            let mut summary_heading = html::content::Heading2::builder();
            summary_heading.text("Seat favourites");
            body.push(summary_heading.build());

            let stats_table = make_stats_table(&constituencies.winning_constituencies);
            body.push(stats_table);

            body.push(html::text_content::ThematicBreak::builder().build());

            let constituency_tables = make_constituency_tables(&constituencies.constituencies);
            body.push(constituency_tables);

            body.script(|script| {
                 script.src("sort.js");
                script.type_("text/javascript");
                return script;
            });

            return body;
        })
        .build();

    return tree.to_string();
}

fn make_constituency_tables(
    constituencies: &Vec<ConstituencyAggregated>,
) -> html::text_content::Division {
    let mut outer_division = html::text_content::Division::builder();
    let mut heading = html::content::Heading2::builder();
    heading.text("Constituencies");
    outer_division.push(heading.build());

    let mut sorters = html::text_content::Division::builder();
    sorters.push(
        html::text_content::Paragraph::builder()
            .text("Sort by: ")
            .build(),
    );
    let mut sorters_list = html::text_content::UnorderedList::builder();

    let make_sorter = |title: &str, key: &str, is_numeric: bool| {
        let anchor = html::inline_text::Anchor::builder()
            .style_attr("cursor: pointer;")
            .data("sort", key.to_string())
            .data("sort-is-numeric", is_numeric.to_string())
            .text(title.to_string())
            .build();
        let list_item = html::text_content::ListItem::builder().push(anchor).build();
        return list_item;
    };

    sorters_list.push(make_sorter("Name", "name", false));
    sorters_list.push(make_sorter("Labour probability", "labourProbability", true));
    sorters_list.push(make_sorter(
        "Conservative probability",
        "conservativeProbability",
        true,
    ));
    sorters_list.push(make_sorter(
        "Lib Dem probability",
        "libDemProbability",
        true,
    ));
    sorters_list.push(make_sorter("Green probability", "greenProbability", true));
    sorters_list.push(make_sorter("Reform probability", "reformProbability", true));
    sorters_list.push(make_sorter("Other probability", "otherProbability", true));
    sorters_list.push(make_sorter("Favourite margin", "favouriteLead", true));
    sorters_list.push(make_sorter(
        "Third place probability",
        "thirdPlaceProbability",
        true,
    ));
    sorters.push(sorters_list.build());
    outer_division.push(sorters.build());

    let mut division = html::text_content::Division::builder();
    division.id("constituencies");
    division.style(
        "display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 1rem;",
    );

    let mut sorted_constituencies = constituencies
        .iter()
        .collect::<Vec<&ConstituencyAggregated>>();
    sorted_constituencies.sort_by(|a, b| a.constituency.cmp(&b.constituency));
    for constituency in &sorted_constituencies {
        let table = make_constituency_table(constituency);
        division.push(table);
    }

    outer_division.push(division.build());

    return outer_division.build();
}

fn make_constituency_table(constituency: &ConstituencyAggregated) -> html::text_content::Division {
    let mut division = html::text_content::Division::builder();

    let mut reverse_sorted_parties = constituency.parties.iter().collect::<Vec<&Party>>();
    reverse_sorted_parties.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap());

    division.data("name", constituency.constituency.clone());
    division.data(
        "labour-probability",
        constituency
            .stats
            .labour_probability
            .map(|p| p.to_string())
            .unwrap_or("".to_string()),
    );
    division.data(
        "conservative-probability",
        constituency
            .stats
            .conservative_probability
            .map(|p| p.to_string())
            .unwrap_or("".to_string()),
    );
    division.data(
        "lib-dem-probability",
        constituency
            .stats
            .lib_dem_probability
            .map(|p| p.to_string())
            .unwrap_or("".to_string()),
    );
    division.data(
        "green-probability",
        constituency
            .stats
            .green_probability
            .map(|p| p.to_string())
            .unwrap_or("".to_string()),
    );
    division.data(
        "reform-probability",
        constituency
            .stats
            .reform_probability
            .map(|p| p.to_string())
            .unwrap_or("".to_string()),
    );
    division.data(
        "other-probability",
        constituency
            .stats
            .other_probability
            .map(|p| p.to_string())
            .unwrap_or("".to_string()),
    );
    division.data(
        "favourite-lead",
        constituency
            .stats
            .favourite_lead
            .map(|p| p.to_string())
            .unwrap_or("".to_string()),
    );
    division.data(
        "third-place-probability",
        constituency
            .stats
            .third_place_probability
            .map(|p| p.to_string())
            .unwrap_or("".to_string()),
    );

    let labour_probability = constituency
        .parties
        .iter()
        .find(|party| party.name == PartyName::Labour)
        .map(|party| party.probability)
        .unwrap_or(0.0);
    division.data("labour-percent", labour_probability.to_string());

    let mut heading = html::content::Heading2::builder();
    heading.text(constituency.constituency.clone());
    division.push(heading.build());

    let mut table = html::tables::Table::builder();
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

fn make_summary_table(summaries: &Vec<MonteCarloSummarySimple>) -> html::tables::Table {
    let mut table = html::tables::Table::builder();
    table.table_row(|row| {
        row.table_header(|header| {
            header.text("Party");
            return header;
        });
        row.table_header(|header| {
            header.text("Median seats");
            header.push(html::inline_text::LineBreak::builder().build());
            header.text("[5th - 95th percentile]");
            return header;
        });
        row.table_header(|header| {
            header.text("Majority percentage");
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
                data.text(summary.median.to_string());
                data.text(" [");
                data.text(summary.lower_5th.to_string());
                data.text(" - ");
                data.text(summary.upper_95th.to_string());
                data.text("]");
                return data;
            })
            .table_cell(|data| {
                data.text(format!("{:.2}%", summary.majority_percentage * 100.0));
                return data;
            })
            .build();
        table.push(row);
    }
    return table.build();
}

fn make_stats_table(stats: &Vec<(PartyName, i32)>) -> html::tables::Table {
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
