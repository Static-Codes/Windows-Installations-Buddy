use reqwest::StatusCode;
use reqwest::blocking::Response;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::error::Error;

const PLATFORM: &str = "amd64";

pub enum ResponseType {
    VecString(Vec<String>),
    StringValue(String),
}

pub fn extract_exe_links_from_html(html: String) -> Vec<String> {
    let document: Html = Html::parse_document(&html); // Parses the provided html string into an Html tree object
    let selector: Selector = Selector::parse("a").unwrap(); // Creates a Selector object to parse all <a> Elements
    document
        .select(&selector) // Returns an Iterator of all <a> Element objects from the parsed tree
        .filter_map(|element| {
            element
                .value()
                .attr("href")
                .filter(|href| href.contains(".exe"))
                .filter(|href| href.contains(PLATFORM))
        }) // Filters the element objects, returning only href links containing ".exe" and the defined PLATFORM "amd64"
        .map(|href: &str| href.to_string()) // Maps the hrefs to a String Iterator
        .collect() // Returns the String Iterator 
}

// This is the rust equivalent to using Union, definitely need to get used to this.

pub fn check_response_for_links(
    response: Result<(StatusCode, String), Box<dyn Error>>,
) -> HashMap<String, ResponseType> {
    let content: String = response.unwrap().1;
    let links: Vec<String> = extract_exe_links_from_html(content);
    let capacity: usize = links.len();
    let mut result: HashMap<String, ResponseType> = HashMap::new();

    if links.is_empty() {
        result.insert(
            "status".to_string(),
            ResponseType::StringValue("no_links_found".to_string()),
        );
        result.insert(
            "error".to_string(),
            ResponseType::StringValue("No links to executable were found".to_string()),
        );
    } else {
        result.insert(
            "status".to_string(),
            ResponseType::StringValue(format!("found_{}_links", capacity.to_string())),
        );
        result.insert("links".to_string(), ResponseType::VecString(links));
    }

    result
}

fn display_startup_message() {
    println!("Welcome to WinStall Buddy v0.0.1\nThis is my first rust project :D");
}

fn temp() -> HashMap<String, ResponseType> {
    let result: Result<(StatusCode, String), Box<dyn Error>> =
        make_web_request("https://www.docker.com/products/docker-desktop/");

    // borrowing is required because the content is partially moved out of result upon the call of match
    match &result {
        // match is a more powerful switch expression
        Ok((_status, _content)) => {
            // println!("Status: {}", status);
            // println!("Content: {}", content);
            let return_value: HashMap<String, ResponseType> = check_response_for_links(result);
            return_value
        }
        Err(e) => {
            let mut return_value: HashMap<String, ResponseType> = HashMap::new();
            return_value.insert(
                "error".to_string(),
                ResponseType::StringValue(e.to_string()),
            );
            return_value
        }
    }
}

fn debug(response: HashMap<String, ResponseType>) {
    for (k, v) in response {
        match v {
            ResponseType::VecString(v) => {
                println!("{}: {:?}", k, v);
            }
            ResponseType::StringValue(v) => {
                println!("{}: {}", k, v);
            }
        }
    }
}
fn main() {
    display_startup_message();
    let response: HashMap<String, ResponseType> = temp();
    debug(response);
}
