use reqwest::{blocking::Client, header};
use serde_json::{Value, json};
use serde::Deserialize;

pub struct Anime {
    pub id: u64,
    pub name: String,
    pub progress: u64,
    pub episodes: u64
}
impl<'de> Deserialize<'de> for Anime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let root: Value = Value::deserialize(deserializer).unwrap();
        Ok(Self {
            id: root["id"].as_u64().unwrap(),
            name: root["media"]["title"]["romaji"].as_str().unwrap().to_string(),
            progress: root["progress"].as_u64().unwrap(),
            episodes: root["media"]["nextAiringEpisode"]["episode"].as_u64().unwrap_or(
                root["media"]["episodes"].as_u64().unwrap_or_default() + 1
            ).saturating_sub(1)
        })
    }
}

#[derive(Clone)]
pub struct User {
    pub client: Client,
    pub token: String,
    pub id: u64
}
impl User {
    pub fn login(auth_code: &str) -> Self {
        let client = Client::new();
        let token = client.post("https://anilist.co/api/v2/oauth/token")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json")
            .json(&json!({
                "grant_type": "authorization_code",
                "client_id": "14244",
                "client_secret": "0wff5vDRO60TB19fStqWGydVtfd01Nahf2TzyXQU",
                "redirect_uri": "ndnd://",
                "code": auth_code
            })).send().unwrap()
            .json::<Value>().unwrap()
            ["access_token"].as_str().unwrap().to_string();
        let id = client.post("https://graphql.anilist.co")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .body(r#"{"query":"{Viewer { id }}"}"#).send().unwrap()
            .json::<Value>().unwrap()
            ["data"]["Viewer"]["id"].as_u64().unwrap();
        Self {
            client, token, id
        }
    }
    pub fn watching(&self) -> Vec<Anime> {
        self.client.post("https://graphql.anilist.co")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.token))
            .body(format!("{{
                \"query\": \"{{
                    MediaListCollection (userId: {}, type: ANIME) {{
                        lists {{
                            name,
                            entries {{
                                id,
                                progress,
                                media {{
                                    title {{
                                        romaji(stylised: false)
                                    }},
                                    episodes,
                                    updatedAt,
                                    nextAiringEpisode {{
                                        airingAt,
                                        episode
                                    }}
                                }}
                            }}
                        }}
                    }}
                }}\"
            }}", self.id).replace("\n", ""))
            .send().unwrap()
            .json::<Value>().unwrap()
            ["data"]["MediaListCollection"]["lists"].as_array().unwrap()
            .into_iter().find(|list| list["name"] == "Watching" ).unwrap()
            ["entries"].as_array().unwrap()
            .into_iter().map(|v|Anime::deserialize(v).unwrap())
            .collect()
    }
    pub fn update_anime_progress(&self, anime: &Anime, progress: i32) {
        self.client.post("https://graphql.anilist.co")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.token))
            .body(format!(
                "{{
                    \"query\": \"mutation {{
                        UpdateMediaListEntries (ids: [{}], progress: {}) {{
                            progress
                        }}
                    }}\"
                }}",
                anime.id,
                anime.progress as i32 + progress
            ).replace("\n", ""))
            .send().unwrap().text().unwrap();
    }
}