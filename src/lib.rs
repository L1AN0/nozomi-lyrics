use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Song {
    pub fn retrieve_163_lyrics(&self) -> Result<Lyrics> {
        static lyrics_url: &str = "http://music.163.com/api/song/lyric";
        let client = reqwest::blocking::Client::default();
        let mut query: HashMap<String, String> = HashMap::default();
        query.insert("id".to_owned(), format!("{}", self.id));
        query.insert("lv".to_owned(), "1".into());
        query.insert("kv".to_owned(), "-1".into());
        query.insert("tv".to_owned(), "-1".into());
        let resp: Lyrics = client.get(lyrics_url).query(&query).send()?.json()?;
        return Ok(resp);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResults {
    pub result: SearchResultsInner,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    id: u64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
    id: u64,
    name: String,
    artists: Vec<Artist>,
    album: Album,
}

impl ToString for Song {
    fn to_string(&self) -> String {
        format!(
            "{} - {:?}",
            self.name,
            self.artists.first().map(|x| &x.name)
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResultsInner {
    pub songs: Vec<Song>,
}

impl PlayingMusic {
    pub fn retrieve_163_search_results(&self) -> SearchResults {
        static search_url: &str = "http://music.163.com/api/search/get";
        let client = reqwest::blocking::Client::default();
        let mut query: HashMap<String, String> = HashMap::default();
        query.insert("s".to_owned(), self.title.clone());
        query.insert("type".to_owned(), "1".into());
        let resp: SearchResults = client
            .post(search_url)
            .query(&query)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return resp;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lyrics {
    pub lrc: LyricsInner,
    pub tlyric: LyricsInner,
}

pub struct PlayingMusic {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    id: u64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LyricsInner {
    lyric: Option<String>,
}

impl LyricsInner {
    pub fn to_lyrics(&self) -> Option<lrc::Lyrics> {
        self.lyric
            .as_ref()
            .map(|x| lrc::Lyrics::from_str(x.as_str()).unwrap())
    }
}
