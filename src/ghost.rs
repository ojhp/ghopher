use chrono::{DateTime, Local};
use serde::Deserialize;

use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct PageListing {
    pub slug: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct PagesResponse {
    pub pages: Vec<PageListing>,
}

#[derive(Debug, Deserialize)]
pub struct PostListing {
    pub slug: String,
    pub title: String,
    pub published_at: DateTime<Local>,
}

#[derive(Debug, Deserialize)]
pub struct PostsResponse {
    pub posts: Vec<PostListing>,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub title: String,
    pub html: String,
}

#[derive(Debug, Deserialize)]
pub struct PageContentResponse {
    pub pages: Vec<Content>,
}

#[derive(Debug, Deserialize)]
pub struct PostContentResponse {
    pub posts: Vec<Content>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct SettingsResponse {
    pub settings: Settings,
}

pub async fn get_pages() -> Result<Vec<PageListing>> {
    let url = format!("{}/api/v3/content/pages/?key={}&fields=slug,title",
        dotenv::var("GHOPHER_API_ROOT")?,
        dotenv::var("GHOPHER_API_KEY")?);

    let response = reqwest::get(url).await?
        .json::<PagesResponse>().await?;

    Ok(response.pages)
}

pub async fn get_posts() -> Result<Vec<PostListing>> {
    let url = format!("{}/api/v3/content/posts/?key={}&fields=slug,title,published_at",
        dotenv::var("GHOPHER_API_ROOT")?,
        dotenv::var("GHOPHER_API_KEY")?);

    let response = reqwest::get(url).await?
        .json::<PostsResponse>().await?;

    Ok(response.posts)
}

pub async fn get_content(slug: &str) -> Result<Content> {
    match get_post(slug).await {
        Ok(content) => Ok(content),
        Err(_) => get_page(slug).await,
    }
}

pub async fn get_page(slug: &str) -> Result<Content> {
    let url = format!("{}/api/v3/content/pages/slug/{}/?key={}&fields=title,html",
        dotenv::var("GHOPHER_API_ROOT")?,
        slug,
        dotenv::var("GHOPHER_API_KEY")?);

    let mut response = reqwest::get(url).await?
        .json::<PageContentResponse>().await?;

    Ok(response.pages.pop().unwrap())
}

pub async fn get_post(slug: &str) -> Result<Content> {
    let url = format!("{}/api/v3/content/posts/slug/{}/?key={}&fields=title,html",
        dotenv::var("GHOPHER_API_ROOT")?,
        slug,
        dotenv::var("GHOPHER_API_KEY")?);

    let mut response = reqwest::get(url).await?
        .json::<PostContentResponse>().await?;

    Ok(response.posts.pop().unwrap())
}

pub async fn get_settings() -> Result<Settings> {
    let url = format!("{}/api/v3/content/settings/?key={}&fields=title,description",
        dotenv::var("GHOPHER_API_ROOT")?,
        dotenv::var("GHOPHER_API_KEY")?);

    let response = reqwest::get(url).await?
        .json::<SettingsResponse>().await?;

    Ok(response.settings)
}
