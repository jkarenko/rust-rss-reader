extern crate reqwest;
extern crate rss;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let feeds_list = read_file("feeds.txt")?;
    let keywords = read_file("keywords.txt")?;
    for feed in feeds_list.iter() {
        println!("\nFeed: {}", feed);
        let rss = get_rss(feed)?;
        let filtered_items = filter_items(rss.items().to_vec(), &keywords);
        print_rss(filtered_items.into_iter().collect());
    }
    Ok(())
}

fn get_rss(url: &str) -> Result<rss::Channel, Box<dyn Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    let channel = rss::Channel::read_from(resp.as_bytes())?;
    Ok(channel)
}

fn read_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = std::fs::read_to_string(filename)?;
    let lines = contents.lines().map(|s| s.to_string()).collect();
    Ok(lines)
}

fn filter_items(items: Vec<rss::Item>, keywords: &Vec<String>) -> Vec<rss::Item> {
    let filtered_items: Vec<_> = items.into_iter().filter(|item| {
        let title = item.title().unwrap_or_default();
        let description = item.description().unwrap_or_default();
        keywords.iter().any(|keyword| {
            title.to_lowercase().contains(keyword) || description.to_lowercase().contains(keyword) || title.to_lowercase().contains(keyword)
        })
    }).collect();
    filtered_items
}

fn print_rss(filtered_items: Vec<rss::Item>) {
    println!("Found {} items", filtered_items.len());
    for item in filtered_items {
        println!("  Date: {:?}", item.pub_date().unwrap_or(""));
        println!("  Title: {:?}", item.title().unwrap_or(""));
        println!("  Description: {:?}", item.description().unwrap_or(""));
        println!("  Link: {:?}\n", item.link().unwrap_or(""));
    }
}
