use ratatui::{Terminal, prelude::Backend};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_xml_rs::from_str;

use crate::{anilist::Anime, utils::{select, download_file}};

#[derive(Deserialize, Debug)]
pub struct Item {
    pub title: String,
    pub link: String,
    pub size: String
}

#[derive(Deserialize, Debug)]
pub struct Channel {
    #[serde(rename = "item", default)]
    pub items: Vec<Item>
}
#[derive(Deserialize, Debug)]
pub struct Rss {
    pub channel: Channel
}

pub fn watch_next_episode<B: Backend>(terminal: &mut Terminal<B>, anime: &Anime) -> bool {
    let search = format!("{} {:02} 1080p", anime.name, anime.progress + 1);
    let res = Client::new().get("https://nyaa.si/?page=rss")
        .query(&[
            ("q", search.as_ref()),
            ("c", "1_2"),
            ("f", "0")
        ])
        .send().unwrap()
        .text().unwrap();
    let rss: Rss = from_str(res.as_ref()).unwrap();
    let items = rss.channel.items.iter().map(|v|v.title.as_str()).collect::<Vec<&str>>();
    if let Some(i) = select(terminal, &anime.name, &items) {
        open::that_in_background(download_file(&rss.channel.items[i].link));
        true
    } else { false }
}