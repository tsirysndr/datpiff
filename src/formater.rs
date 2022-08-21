use colored_json::ToColoredJson;

use crate::types::Mixtape;

pub fn format_results(results: Vec<Mixtape>) {
    println!(
        "{}",
        serde_json::to_string(&results)
            .unwrap()
            .to_colored_json_auto()
            .unwrap()
    );
}
