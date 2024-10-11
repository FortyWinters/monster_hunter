use anyhow::Error;
use anyhow::Ok;
use reqwest::Client;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Spider {
    client: Client,
    world_url: String,
    rise_url: String,
    _wild_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpMonster {
    pub monster_id: i32,
    pub monster_name: String,
    pub monster_type: i32,
    pub monster_description: String,
    pub monster_icon_url: String,
    pub game_type: i32,
}

#[allow(dead_code)]
impl Spider {
    pub fn new() -> Result<Spider, Error> {
        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        Ok(Spider {
            client,
            world_url: "https://mhworld.kiranico.com/zh".to_string(),
            rise_url: "https://mhrise.kiranico.com/zh".to_string(),
            _wild_url: "".to_string(),
        })
    }

    async fn request_html(&self, url: &str) -> Result<Document, Error> {
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        return Ok(Document::from(body.as_str()));
    }

    pub async fn get_world_monster_url(&self) -> Result<Vec<String>, Error> {
        let url = format!("{}/monsters", self.world_url);
        let document = self.request_html(&url).await?;

        let mut monster_url_vec: Vec<String> = Vec::new();
        for node in document.find(Name("div").and(Attr("class", "d-flex justify-content-between")))
        {
            if let Some(link) = node.find(Name("a")).next() {
                if let Some(href) = link.attr("href") {
                    if let Some(pos) = href.find("/monsters") {
                        let trimmed_url = &href[pos..];
                        monster_url_vec.push(trimmed_url.to_string());
                    }
                }
            }
        }
        Ok(monster_url_vec)
    }

    pub async fn get_world_monster_by_url(
        &self,
        url_vec: Vec<String>,
    ) -> Result<Vec<SpMonster>, Error> {
        let mut monster_info_vec: Vec<SpMonster> = Vec::new();
        let mut monster_id_counter = 0;

        for url in url_vec {
            let full_url = format!("{}{}", self.world_url, url);
            let document = self.request_html(&full_url).await?;

            let monster_name = document
                .find(Name("meta").and(Attr("property", "og:title")))
                .next()
                .and_then(|meta| meta.attr("content"))
                .unwrap_or("");

            let monster_description = document
                .find(Name("meta").and(Attr("property", "og:description")))
                .next()
                .and_then(|meta| meta.attr("content"))
                .unwrap_or("");

            let monster_icon_url = document
                .find(Name("meta").and(Attr("property", "og:image")))
                .next()
                .and_then(|meta| meta.attr("content"))
                .unwrap_or("");

            let monster_info = SpMonster {
                monster_id: monster_id_counter,
                monster_name: monster_name.to_string(),
                monster_type: 0,
                monster_description: monster_description.to_string(),
                monster_icon_url: monster_icon_url.to_string(),
                game_type: 0,
            };
            monster_info_vec.push(monster_info);

            monster_id_counter += 1;
        }

        Ok(monster_info_vec)
    }

    pub async fn get_rise_monster_url(&self, size: &str) -> Result<Vec<(String, String)>, Error> {
        let url = format!("{}/data/monsters?view={}", self.rise_url, size);
        let document = self.request_html(&url).await?;

        let mut monster_url_vec: Vec<String> = Vec::new();
        let mut monster_icon_vec: Vec<String> = Vec::new();

        for node in document.find(Name("div").and(Attr("class", "rounded-lg overflow-hidden bg-gray-200 dark:bg-gray-800 aspect-w-1 aspect-h-1 group-hover:opacity-75"))) {
            if let Some(img_node) = node.find(Name("img")).next() {
                if let Some(img_src) = img_node.attr("src") {
                    monster_icon_vec.push(img_src.to_string());
                }
            }
        }

        for node in document.find(Name("h3").and(Attr(
            "class",
            "text-sm font-medium text-gray-900 dark:text-gray-300",
        ))) {
            if let Some(link_node) = node.find(Name("a")).next() {
                if let Some(href) = link_node.attr("href") {
                    monster_url_vec.push(href.to_string());
                }
            }
        }

        let mut monster_url_icon_vec: Vec<(String, String)> = Vec::new();
        let count = monster_icon_vec.len().min(monster_url_vec.len());
        for i in 0..count {
            monster_url_icon_vec.push((monster_icon_vec[i].clone(), monster_url_vec[i].clone()));
        }

        Ok(monster_url_icon_vec)
    }

    pub async fn get_rise_monster_by_url(
        &self,
        url_vec: Vec<(String, String)>,
    ) -> Result<Vec<SpMonster>, Error> {
        let mut monster_info_vec: Vec<SpMonster> = Vec::new();
        let mut monster_id_counter = 1000;

        for (icon_url, monster_url) in url_vec {
            let document = self.request_html(&monster_url).await?;

            let _monster_name = document
                .find(Name("meta").and(Attr("property", "og:title")))
                .next()
                .and_then(|meta| meta.attr("content"))
                .unwrap_or("");

            let monster_name: String = _monster_name
                .to_string()
                .split('|')
                .next()
                .map(|name| name.trim().to_string())
                .unwrap_or_else(String::new);

            let monster_description = document
                .find(Name("meta").and(Attr("property", "og:description")))
                .next()
                .and_then(|meta| meta.attr("content"))
                .unwrap_or("");

            let monster_info = SpMonster {
                monster_id: monster_id_counter,
                monster_name: monster_name,
                monster_type: 0,
                monster_description: monster_description.to_string(),
                monster_icon_url: icon_url,
                game_type: 1,
            };
            monster_info_vec.push(monster_info);

            monster_id_counter += 1;
        }

        Ok(monster_info_vec)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_world_monster() {
        let spider = Spider::new().unwrap();

        let monster_url_img_vec = spider.get_rise_monster_url("lg").await.unwrap();
        let monster_url_img_vec = spider
            .get_rise_monster_by_url(monster_url_img_vec)
            .await
            .unwrap();
        println!("{:?}", monster_url_img_vec);
    }
}
