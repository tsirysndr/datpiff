use clap::{arg, Arg, Command};
use colored_json::ToColoredJson;
use datpiff::{formater::format_results, parser::Parser};

fn cli() -> Command<'static> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    Command::new("datpiff")
        .version(VERSION)
        .author("Tsiry Sandratraina <tsiry.sndr@aol.com>")
        .about(
            r#"

   _____        _         _  __  __ 
  |  __ \      | |       (_)/ _|/ _|
  | |  | | __ _| |_ _ __  _| |_| |_ 
  | |  | |/ _` | __| '_ \| |  _|  _|
  | |__| | (_| | |_| |_) | | | | |  
  |_____/ \__,_|\__| .__/|_|_| |_|  
                   | |              
                   |_|              
 
Scrapes the datpiff website for the latest mixtapes"#,
        )
        .subcommand_required(true)
        .subcommand(Command::new("latest").about("Get the latest mixtapes"))
        .subcommand(Command::new("hot").about("Get the hottest mixtapes"))
        .subcommand(Command::new("exclusives").about("Get the latest exclusives"))
        .subcommand(Command::new("top").about("Get the top mixtapes of the month"))
        .subcommand(
            Command::new("search").about("Search mixtape").arg(
                Arg::with_name("query")
                    .help("The query to search for")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("info")
                .about("Show details about a mixtape")
                .arg(
                    Arg::with_name("id")
                        .help("The id of the mixtape")
                        .required(true)
                        .index(1),
                ),
        )
        .arg(arg!(-j --json ... "Output results in json format").required(false))
}

#[tokio::main]
async fn main() -> Result<(), surf::Error> {
    let matches = cli().get_matches();
    let parser = Parser::new();

    let json = matches.is_present("json");

    match matches.subcommand() {
        Some(("exclusives", _)) => format_results(parser.get_exclusive_mixtapes().await?, json),
        Some(("hot", _)) => format_results(parser.get_hot_mixtapes().await?, json),
        Some(("latest", _)) => format_results(parser.get_latest_mixtapes().await?, json),
        Some(("search", sub_matches)) => format_results(
            parser
                .search_mixtapes(sub_matches.get_one::<String>("query").unwrap())
                .await?,
            json,
        ),
        Some(("top", _)) => format_results(parser.get_top_mixtapes().await?, json),
        Some(("info", sub_matches)) => {
            let mixtape = parser
                .get_mixtape(sub_matches.get_one::<String>("id").unwrap())
                .await?;
            println!(
                "{}",
                serde_json::to_string(&mixtape)
                    .unwrap()
                    .to_colored_json_auto()
                    .unwrap()
            );
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    };

    Ok(())
}
