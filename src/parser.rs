use std::time::Duration;

use scraper::{Html, Selector};
use surf::{Client, Config, Url};
use urlencoding::encode;

use crate::{constants::BASE_URL, types::Mixtape};

pub struct Parser {
    client: Client,
}

impl Parser {
    pub fn new() -> Self {
        let client: Client = Config::new()
            .set_base_url(Url::parse(BASE_URL).unwrap())
            .set_timeout(Some(Duration::from_secs(5)))
            .try_into()
            .unwrap();
        Self { client }
    }
    pub async fn get_latest_mixtapes(&self) -> Result<Vec<Mixtape>, surf::Error> {
        let mut mixtapes: Vec<Mixtape> = Vec::new();
        let page = self.client.get("/").recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse(".contentItemInner").unwrap();
        for (i, element) in document.select(&selector).enumerate() {
            let div = element.select(&Selector::parse(".artist").unwrap()).next();
            let a = element.select(&Selector::parse(".title").unwrap()).next();
            let span = element.select(&Selector::parse("span").unwrap()).next();
            if div == None || a == None || i > 7 {
                continue;
            }
            let artist = div.unwrap().text().collect::<Vec<_>>()[0];
            let title = a.unwrap().text().collect::<Vec<_>>()[0];
            let mut link = element
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string();
            link.remove(0);
            let listens = span.unwrap().text().collect::<Vec<_>>()[0];
            mixtapes.push(Mixtape {
                artist: artist.to_string(),
                title: title.to_string(),
                listens: listens.to_string(),
                link: format!("{}{}", BASE_URL, link),
            })
        }
        Ok(mixtapes)
    }

    pub async fn get_hot_mixtapes(&self) -> Result<Vec<Mixtape>, surf::Error> {
        let mut mixtapes: Vec<Mixtape> = Vec::new();
        let page = self.client.get("/").recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse(".contentItemInner").unwrap();
        for (i, element) in document.select(&selector).enumerate() {
            let div = element.select(&Selector::parse(".artist").unwrap()).next();
            let a = element.select(&Selector::parse(".title").unwrap()).next();
            let span = element.select(&Selector::parse("span").unwrap()).next();
            if div == None || a == None || i < 8 || i > 15 {
                continue;
            }
            let artist = div.unwrap().text().collect::<Vec<_>>()[0];
            let title = a.unwrap().text().collect::<Vec<_>>()[0];
            let mut link = element
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string();
            link.remove(0);
            let listens = span.unwrap().text().collect::<Vec<_>>()[0];
            mixtapes.push(Mixtape {
                title: title.to_string(),
                artist: artist.to_string(),
                listens: listens.to_string(),
                link: format!("{}{}", BASE_URL, link),
            })
        }
        Ok(mixtapes)
    }

    pub async fn get_exclusive_mixtapes(&self) -> Result<Vec<Mixtape>, surf::Error> {
        let mut mixtapes: Vec<Mixtape> = Vec::new();
        let page = self.client.get("/").recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse(".contentItemInner").unwrap();
        for (i, element) in document.select(&selector).enumerate() {
            let div = element.select(&Selector::parse(".artist").unwrap()).next();
            let a = element.select(&Selector::parse(".title").unwrap()).next();
            let span = element.select(&Selector::parse("span").unwrap()).next();
            if div == None || a == None || i < 16 || i > 19 {
                continue;
            }
            let artist = div.unwrap().text().collect::<Vec<_>>()[0];
            let title = a.unwrap().text().collect::<Vec<_>>()[0];
            let mut link = element
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string();
            link.remove(0);
            let listens = span.unwrap().text().collect::<Vec<_>>()[0];

            mixtapes.push(Mixtape {
                title: title.to_string(),
                artist: artist.to_string(),
                listens: listens.to_string(),
                link: format!("{}{}", BASE_URL, link),
            })
        }
        Ok(mixtapes)
    }

    pub async fn get_top_mixtapes(&self) -> Result<Vec<Mixtape>, surf::Error> {
        let mut mixtapes: Vec<Mixtape> = Vec::new();
        let page = self.client.get("/").recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse(".contentItemInner").unwrap();
        for (i, element) in document.select(&selector).enumerate() {
            let div = element.select(&Selector::parse(".artist").unwrap()).next();
            let a = element.select(&Selector::parse(".title").unwrap()).next();
            let span = element.select(&Selector::parse("span").unwrap()).next();
            if div == None || a == None || i < 28 {
                continue;
            }
            let artist = div.unwrap().text().collect::<Vec<_>>()[0];
            let title = a.unwrap().text().collect::<Vec<_>>()[0];
            let mut link = element
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string();
            link.remove(0);
            let listens = span.unwrap().text().collect::<Vec<_>>()[0];
            mixtapes.push(Mixtape {
                artist: artist.to_string(),
                title: title.to_string(),
                listens: listens.to_string(),
                link: format!("{}{}", BASE_URL, link),
            })
        }
        Ok(mixtapes)
    }

    pub async fn search_mixtapes(&self, query: &str) -> Result<Vec<Mixtape>, surf::Error> {
        let criteria = encode(query);
        let mut mixtapes: Vec<Mixtape> = Vec::new();
        let page = self
            .client
            .get(format!("/mixtapes-search.php?criteria={}", criteria))
            .recv_string()
            .await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse(".contentItemInner").unwrap();
        for element in document.select(&selector) {
            let div = element.select(&Selector::parse(".artist").unwrap()).next();
            let a = element.select(&Selector::parse(".title").unwrap()).next();
            let span = element.select(&Selector::parse("span").unwrap()).next();
            if div == None || a == None {
                continue;
            }
            let artist = div.unwrap().text().collect::<Vec<_>>()[0];
            let title = a.unwrap().text().collect::<Vec<_>>()[0];
            let mut link = element
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string();
            link.remove(0);
            let listens = span.unwrap().text().collect::<Vec<_>>()[0];
            mixtapes.push(Mixtape {
                title: title.to_string(),
                artist: artist.to_string(),
                listens: listens.to_string(),
                link: format!("{}{}", BASE_URL, link),
            })
        }
        Ok(mixtapes)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
