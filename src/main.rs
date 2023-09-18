extern crate reqwest;
extern crate rss;

use std::error::Error;
use serde::Serialize;
use warp::{Filter};
use log::{info, error};
use serde_json::json;
use futures::stream::{FuturesUnordered, StreamExt};

#[derive(Serialize)]
struct SerializableItem {
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    pub_date: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting server");
    let rss_route = warp::path("rss")
        .and(warp::get())
        .and_then(rss_reader);

    let routes = rss_route;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
async fn rss_reader() -> Result<impl warp::Reply, warp::Rejection> {
    let feeds_list = read_file("feeds.txt").await.map_err(|_| warp::reject())?;
    let keywords = read_file("keywords.txt").await.map_err(|_| warp::reject())?;
    let mut all_filtered_items = Vec::new();

    let mut futures = FuturesUnordered::new();

    for feed in feeds_list.iter() {
        futures.push(get_rss(feed));
    }

    while let Some(result) = futures.next().await {
        match result {
            Ok(rss) => {
                let filtered_items = filter_items(rss.items().to_vec(), &keywords);
                info!("Fetched RSS: {}: {} matches", rss.title(), filtered_items.len());
                all_filtered_items.extend(filtered_items);
            }
            Err(e) => {
                error!("Failed to fetch RSS: {}", e);
            }
        }
    }
    // print_rss(&all_filtered_items);

    let serializable_items: Vec<SerializableItem> = all_filtered_items
        .into_iter()
        .map(|item| SerializableItem {
            title: item.title().map(String::from),
            link: item.link().map(String::from),
            description: item.description().map(String::from),
            pub_date: item.pub_date().map(String::from),
        })
        .collect();

    let json_data = json!(serializable_items);  // Serialize using serde_json
    Ok(warp::reply::json(&json_data))  // Return as JSON reply
}


async fn get_rss(url: &str) -> Result<rss::Channel, Box<dyn Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    let channel = rss::Channel::read_from(resp.as_bytes())?;
    Ok(channel)
}

async fn read_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = tokio::fs::read_to_string(filename).await?;
    let lines = contents.lines().map(|s| s.to_string()).collect();
    Ok(lines)
}

fn filter_items(items: Vec<rss::Item>, keywords: &Vec<String>) -> Vec<rss::Item> {
    let filtered_items: Vec<_> = items.into_iter().filter(|item| {
        let title = item.title().unwrap_or_default();
        let description = item.description().unwrap_or_default();
        keywords.iter().any(|keyword| {
            title.to_lowercase().contains(keyword) || description.to_lowercase().contains(keyword)
        })
    }).collect();
    filtered_items
}

fn print_rss(filtered_items: &Vec<rss::Item>) {
    println!("Found {} items", filtered_items.len());
    for item in filtered_items {
        println!("  Date: {:?}", item.pub_date().unwrap_or(""));
        println!("  Title: {:?}", item.title().unwrap_or(""));
        println!("  Description: {:?}", item.description().unwrap_or(""));
        println!("  Link: {:?}\n", item.link().unwrap_or(""));
    }
}
