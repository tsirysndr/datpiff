use colored_json::ToColoredJson;
use tabled::{object::Columns, Modify, Style, Table, Width};

use crate::types::Mixtape;

pub fn format_results(results: Vec<Mixtape>, json: bool) {
    if results.len() == 0 {
        println!("No mixtapes found.");
        return;
    }
    if json {
        println!(
            "{}",
            serde_json::to_string(&results)
                .unwrap()
                .to_colored_json_auto()
                .unwrap()
        );
        return;
    }
    println!(
        "{}",
        Table::new(results)
            .with(Modify::new(Columns::single(0)).with(Width::truncate(10000)))
            .with(Modify::new(Columns::single(1)).with(Width::truncate(50).suffix("...")))
            .with(Style::psql())
    );
}
