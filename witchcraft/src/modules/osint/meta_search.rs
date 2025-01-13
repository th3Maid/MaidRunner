use crate::core::consts::SW_DEBUG;
use crate::core::core::*;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use reqwest::blocking::Client;
use serde_json;
use std::{fs, time::Duration};

#[derive(serde::Deserialize)]
pub struct OsintEntry {
    pub url: String,
    pub category: String,
    pub global_rank: u32,
    pub country: String,
    pub nsfw: String,
    pub match_positive: Vec<String>,
    pub match_negative: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct OsintDatabase {
    pub index: Vec<OsintEntry>,
}

pub fn read_json_file(file_path: &str) -> OsintDatabase {
    let file_content =
        fs::read_to_string(file_path).expect("read_json_file :: Failed to read file");
    let data: OsintDatabase =
        serde_json::from_str(&file_content).expect("read_json_file :: Failed to parse JSON");
    data
}

fn grep_page_content(url: String) -> String {
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .window_size(Some((1920, 1080)))
            .build()
            .unwrap(),
    )
    .unwrap();

    let tab = browser.new_tab().unwrap();

    match tab.navigate_to(&url) {
        Ok(_) => {}
        Err(err) => {
            raise(
                &format!("Failed to navigate to URL: {}", err.to_string()),
                "fail",
            );
            return String::new();
        }
    }

    match tab.wait_until_navigated() {
        Ok(_) => {}
        Err(err) => {
            raise(&format!("exec_meta_search :: {}", err.to_string()), "fail");
            return String::new();
        }
    }

    match tab.wait_for_element_with_custom_timeout("body", Duration::from_secs(15)) {
        Ok(_element) => {
            // element.click().unwrap(); // Example action
        }
        Err(err) => {
            raise(
                &format!("Element was not found within timeout: {}", err.to_string()),
                "fail",
            );
            return String::new();
        }
    }

    let content = tab.get_content().unwrap();
    return content;
}

pub fn exec_meta_search(data: OsintEntry, keyword: &str) {
    fn filter(data: OsintEntry, content: String, keyword: &str) {
        let mut positive_found: Vec<String> = Vec::new();
        let mut negative_found: Vec<String> = Vec::new();

        if data.match_negative.is_empty() {
            raise("Negative patterns can't be empty!", "fail");
        }

        for ps in &data.match_positive {
            if content.to_lowercase().contains(&ps.to_lowercase()) {
                positive_found.push(ps.to_string());
            }
        }

        for ng in &data.match_negative {
            if content.to_lowercase().contains(&ng.to_lowercase()) {
                negative_found.push(ng.to_string());
            }
        }

        if negative_found.is_empty() {
            let key = data.url.replace("@@keyword", keyword);
            raise(&format!("Found! :: {}", key), "good");
            raise("Adicional information", "none");
            raise(&format!("\t Category: {}", data.category), "none");
            raise(&format!("\t Global rank: {}", data.global_rank), "none");
            raise(&format!("\t Country: {}", data.country), "none");
            raise(&format!("\t Is nsfw: {}\n", data.nsfw), "none");
        }
    }

    let url = &data.url.replace("@@keyword", &keyword);
    let client = Client::new();

    match client.get(url).send() {
        Ok(res) => {
            if res.status().as_u16() == 200 {
                let content = grep_page_content(url.to_string());
                filter(data, content, keyword);
            }
        }
        Err(err) => {
            if SW_DEBUG {
                raise(
                    &format!("exec_meta_search :: URL :: {} \n {}", url, err.to_string()),
                    "fail",
                );
            }
            return;
        }
    }
}

pub fn social_links(argsv: &[String]) -> i32 {
    let keyword = search_value("keyword", argsv);
    let data = read_json_file(&get_spellbook_path("osint/osintdb.json"));

    raise("Start scanning, this will take a while...\n", "message");
    for dt in data.index {
        exec_meta_search(dt, &keyword);
    }

    return 0;
}
