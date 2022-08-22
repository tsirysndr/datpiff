use clap::{Arg, Command};
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
}

#[tokio::main]
async fn main() -> Result<(), surf::Error> {
    let matches = cli().get_matches();
    let parser = Parser::new();

    match matches.subcommand() {
        Some(("exclusives", _)) => format_results(parser.get_exclusive_mixtapes().await?),
        Some(("hot", _)) => format_results(parser.get_hot_mixtapes().await?),
        Some(("latest", _)) => format_results(parser.get_latest_mixtapes().await?),
        Some(("search", sub_matches)) => format_results(
            parser
                .search_mixtapes(sub_matches.get_one::<String>("query").unwrap())
                .await?,
        ),
        Some(("top", _)) => format_results(parser.get_top_mixtapes().await?),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    };

    Ok(())
}
