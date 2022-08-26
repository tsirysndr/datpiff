use std::time::Duration;

use scraper::{ElementRef, Html, Selector};
use surf::{Client, Config, Url};
use urlencoding::encode;

use crate::{
    constants::BASE_URL,
    types::{Mixtape, MixtapeDetails},
};

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

    pub async fn get_mixtape(&self, id: &str) -> Result<MixtapeDetails, surf::Error> {
        let page = self
            .client
            .get(format!("{}/{}.html", BASE_URL, id))
            .recv_string()
            .await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse(".tracklist").unwrap();
        let ul = document.select(&selector).next().unwrap();
        let mut tracks: Vec<String> = Vec::new();

        for (i, item) in ul
            .select(&Selector::parse(".trackTitle").unwrap())
            .map(|title| title.text().next().unwrap().to_string())
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
        {
            tracks.push(format!("{}. {}", i + 1, item));
        }

        let upload = self.parse_uploader(&document);
        let details = self.parse_mixtape_details(&document);

        Ok(MixtapeDetails {
            tracks,
            uploader: upload.uploader,
            added_at: upload.added_at,
            artist: details.artist,
            title: details.title,
            dj: details.dj,
            listens: details.listens,
        })
    }

    fn parse_uploader(&self, element: &Html) -> MixtapeDetails {
        let bar = element
            .select(&Selector::parse(".awardsBar").unwrap())
            .next()
            .unwrap();
        let by = bar
            .select(&Selector::parse("a").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();
        let added_at = bar
            .select(&Selector::parse(".charcoal").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();
        MixtapeDetails {
            uploader: by.to_string(),
            added_at: added_at.to_string(),
            artist: "".to_string(),
            title: "".to_string(),
            dj: "".to_string(),
            listens: "".to_string(),
            tracks: Vec::new(),
        }
    }

    fn parse_mixtape_details(&self, document: &Html) -> MixtapeDetails {
        let details = document
            .select(&Selector::parse(".tapeDetails").unwrap())
            .next()
            .unwrap();

        let title = details
            .select(&Selector::parse(".title").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string();
        let artist = details
            .select(&Selector::parse(".artist").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string();
        let dj = details
            .select(&Selector::parse(".dj").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string();
        let listens = details
            .select(&Selector::parse(".listens").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string();
        MixtapeDetails {
            tracks: Vec::new(),
            uploader: String::new(),
            added_at: String::new(),
            artist,
            title,
            dj,
            listens,
        }
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
            id: link.replace("/", "").replace(
                ".html,
            tracks: todo!(), ",
                "",
            ),
            artist: artist.to_string(),
            title: title.to_string(),
            listens: listens.to_string(),
            link: format!("{}/{}", BASE_URL, link),
            cover: format!("http:{}", cover),
        });
    }
}
