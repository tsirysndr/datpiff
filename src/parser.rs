use std::time::Duration;

use scraper::{ElementRef, Html, Selector};
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
            if i > 7 {
                continue;
            }
            let mixtape = self.parse_mixtape(&element);
            if mixtape.is_some() {
                mixtapes.push(mixtape.unwrap());
            }
        }
        Ok(mixtapes)
    }

    pub async fn get_hot_mixtapes(&self) -> Result<Vec<Mixtape>, surf::Error> {
        let mut mixtapes: Vec<Mixtape> = Vec::new();
        let page = self.client.get("/").recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse(".contentItemInner").unwrap();
        for (i, element) in document.select(&selector).enumerate() {
            if i < 8 || i > 15 {
                continue;
            }
            let mixtape = self.parse_mixtape(&element);
            if mixtape.is_some() {
                mixtapes.push(mixtape.unwrap());
            }
        }
        Ok(mixtapes)
    }

    pub async fn get_exclusive_mixtapes(&self) -> Result<Vec<Mixtape>, surf::Error> {
        let mut mixtapes: Vec<Mixtape> = Vec::new();
        let page = self.client.get("/").recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse(".contentItemInner").unwrap();
        for (i, element) in document.select(&selector).enumerate() {
            if i < 16 || i > 19 {
                continue;
            }
            let mixtape = self.parse_mixtape(&element);
            if mixtape.is_some() {
                mixtapes.push(mixtape.unwrap());
            }
        }
        Ok(mixtapes)
    }

    pub async fn get_top_mixtapes(&self) -> Result<Vec<Mixtape>, surf::Error> {
        let mut mixtapes: Vec<Mixtape> = Vec::new();
        let page = self.client.get("/").recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse(".contentItemInner").unwrap();
        for (i, element) in document.select(&selector).enumerate() {
            if i < 28 {
                continue;
            }
            let mixtape = self.parse_mixtape(&element);
            if mixtape.is_some() {
                mixtapes.push(mixtape.unwrap());
            }
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
            let mixtape = self.parse_mixtape(&element);
            if mixtape.is_some() {
                mixtapes.push(mixtape.unwrap());
            }
        }
        Ok(mixtapes)
    }

    fn parse_mixtape(&self, element: &ElementRef) -> Option<Mixtape> {
        let div = element.select(&Selector::parse(".artist").unwrap()).next();
        let a = element.select(&Selector::parse(".title").unwrap()).next();
        let span = element.select(&Selector::parse("span").unwrap()).next();
        if div == None || a == None {
            return None;
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

        let cover = element
            .select(&Selector::parse(r#"img[alt="Mixtape Cover"]"#).unwrap())
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap()
            .to_string();

        return Some(Mixtape {
            id: link.replace("/", "").replace(".html", ""),
            artist: artist.to_string(),
            title: title.to_string(),
            listens: listens.to_string(),
            link: format!("{}/{}", BASE_URL, link),
            cover: format!("http:{}", cover),
        });
    }
}
