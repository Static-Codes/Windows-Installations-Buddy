use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn _debug(response: Vec<String>) {
    for v in response {
        println!("{}", v);
    }
}

fn _debug_json(obj: Value) {
    println!("{}", to_string_pretty(&obj).unwrap());
}

fn _extract_release_version_from_github_title(html: &str) -> Option<String> {
    // Compiles a new pattern, and handles errors through unwrap
    let regex = Regex::new(r"<title>Release Release v(\d+\.\d+\.\d+)").unwrap();

    // Some(caps) is the rust equivalent of Union[type, None] from python
    if let Some(caps) = regex.captures(html) {
        Some(caps[0].to_string())
    } else {
        Some("No release version found.".to_string()) // Gross solution to handle the return a static string
    }
}

fn _write_file(filename: &str, contents: Value) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename).unwrap();
    file.write_all(contents.to_string().as_bytes())?;
    Ok(())
}
