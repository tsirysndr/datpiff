use std::time::Duration;

use clap::{Arg, Command};
use datpiff::constants::BASE_URL;
use scraper::{Html, Selector};
use surf::{Client, Config, Url};
use urlencoding::encode;

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
            Command::new("search").about("Search torrents").arg(
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
    let client: Client = Config::new()
        .set_base_url(Url::parse(BASE_URL)?)
        .set_timeout(Some(Duration::from_secs(5)))
        .try_into()
        .unwrap();

    match matches.subcommand() {
        Some(("exclusives", _)) => get_exclusive_mixtapes(&client).await?,
        Some(("hot", _)) => get_hot_mixtapes(&client).await?,
        Some(("latest", _)) => get_latest_mixtapes(&client).await?,
        Some(("search", sub_matches)) => {
            search_mixtapes(&client, sub_matches.get_one::<String>("query").unwrap()).await?
        }
        Some(("top", _)) => get_top_mixtapes(&client).await?,
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}

async fn get_latest_mixtapes(client: &Client) -> Result<(), surf::Error> {
    let page = client.get("/").recv_string().await?;
    let document = Html::parse_document(&page);
    let selector = Selector::parse(".contentItemInner").unwrap();
    for (i, element) in document.select(&selector).enumerate() {
        let div = element.select(&Selector::parse(".artist").unwrap()).next();
        let a = element.select(&Selector::parse(".title").unwrap()).next();
        if div == None || a == None || i > 7 {
            continue;
        }
        let artist = div.unwrap().text().collect::<Vec<_>>()[0];
        let title = a.unwrap().text().collect::<Vec<_>>()[0];
        println!("{} - {}", artist, title);
    }
    Ok(())
}

async fn get_hot_mixtapes(client: &Client) -> Result<(), surf::Error> {
    let page = client.get("/").recv_string().await?;
    let document = Html::parse_document(&page);
    let selector = Selector::parse(".contentItemInner").unwrap();
    for (i, element) in document.select(&selector).enumerate() {
        let div = element.select(&Selector::parse(".artist").unwrap()).next();
        let a = element.select(&Selector::parse(".title").unwrap()).next();
        if div == None || a == None || i < 8 || i > 15 {
            continue;
        }
        let artist = div.unwrap().text().collect::<Vec<_>>()[0];
        let title = a.unwrap().text().collect::<Vec<_>>()[0];
        println!("{} - {}", artist, title);
    }
    Ok(())
}

async fn get_exclusive_mixtapes(client: &Client) -> Result<(), surf::Error> {
    let page = client.get("/").recv_string().await?;
    let document = Html::parse_document(&page);
    let selector = Selector::parse(".contentItemInner").unwrap();
    for (i, element) in document.select(&selector).enumerate() {
        let div = element.select(&Selector::parse(".artist").unwrap()).next();
        let a = element.select(&Selector::parse(".title").unwrap()).next();
        if div == None || a == None || i < 16 || i > 19 {
            continue;
        }
        let artist = div.unwrap().text().collect::<Vec<_>>()[0];
        let title = a.unwrap().text().collect::<Vec<_>>()[0];
        println!("{} - {}", artist, title);
    }
    Ok(())
}

async fn get_top_mixtapes(client: &Client) -> Result<(), surf::Error> {
    let page = client.get("/").recv_string().await?;
    let document = Html::parse_document(&page);
    let selector = Selector::parse(".contentItemInner").unwrap();
    for (i, element) in document.select(&selector).enumerate() {
        let div = element.select(&Selector::parse(".artist").unwrap()).next();
        let a = element.select(&Selector::parse(".title").unwrap()).next();
        if div == None || a == None || i < 28 {
            continue;
        }
        let artist = div.unwrap().text().collect::<Vec<_>>()[0];
        let title = a.unwrap().text().collect::<Vec<_>>()[0];
        println!("{} - {}", artist, title);
    }
    Ok(())
}

async fn search_mixtapes(client: &Client, query: &str) -> Result<(), surf::Error> {
    let criteria = encode(query);
    let page = client
        .get(format!("/mixtapes-search.php?criteria={}", criteria))
        .recv_string()
        .await?;
    let document = Html::parse_document(&page);
    let selector = Selector::parse(".contentItemInner").unwrap();
    for element in document.select(&selector) {
        let div = element.select(&Selector::parse(".artist").unwrap()).next();
        let a = element.select(&Selector::parse(".title").unwrap()).next();
        if div == None || a == None {
            continue;
        }
        let artist = div.unwrap().text().collect::<Vec<_>>()[0];
        let title = a.unwrap().text().collect::<Vec<_>>()[0];
        println!("{} - {}", artist, title);
    }
    Ok(())
}
