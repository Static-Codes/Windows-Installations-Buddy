use crate::cmd;
use crate::links;
use ini::{Ini, Properties};
use reqwest::StatusCode;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::{collections::HashMap, env, fs, process::exit};
use url::Url;
use urlencoding::decode;

#[allow(dead_code)]
pub fn make_web_request_w_ua(
    url: &str,
) -> Result<(StatusCode, Vec<u8>, HeaderMap), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();

    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"));
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    let timeout = Duration::from_secs(120);
    let client = Client::builder()
        .timeout(timeout)
        .default_headers(headers)
        .build()?;

    let response = client.get(url).send().unwrap();

    println!("{}", response.url());
    let mut resp_headers = response.headers().clone();

    let final_url = response.url().as_str().to_string();
    let value = String::from("/") + &final_url;
    let header_value = HeaderValue::from_str(&value).unwrap();

    resp_headers.append("location", header_value); // Clone is required to convert a &HeaderMap to HeaderMap
    let status = response.status();
    if status.is_success() {
        let body = response.bytes()?.to_vec(); // response.text() is only valid for utf-8 encoded files (which executables are not)
        Ok((status, body, resp_headers))
    } else {
        let status: StatusCode = response.status();
        if status.as_u16() == 410 {
            return Ok((status, vec![], HeaderMap::new()));
        }
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Request failed with status {}, please try again", status),
        )))
    }
}

#[allow(dead_code)]

pub enum ValidStarterArg {
    DOWNLOAD,
    HELP,
    INSTALL,
    VERSION,
}

#[allow(dead_code)]
pub fn map_enum_to_arg_str(arg: ValidStarterArg) -> String {
    match arg {
        ValidStarterArg::DOWNLOAD => String::from("download"),
        ValidStarterArg::HELP => String::from("help"),
        ValidStarterArg::INSTALL => String::from("install"),
        ValidStarterArg::VERSION => String::from("version"),
    }
}

pub fn map_arg_str_to_enum(arg: String) -> ValidStarterArg {
    // Deferencing is required because the match expression doesnt accept
    match &*arg {
        "download" => ValidStarterArg::DOWNLOAD,
        "help" => ValidStarterArg::HELP,
        "install" => ValidStarterArg::INSTALL,
        "version" => ValidStarterArg::VERSION,
        _ => ValidStarterArg::HELP, // In the event an invalid arg is returned, the help key is returned
    }
}

#[allow(unused_assignments)]
#[allow(dead_code)]
pub fn handle_starter_arg(arg: ValidStarterArg) {
    match arg {
        ValidStarterArg::DOWNLOAD => {
            println!("Loading download configuration, please wait...");
            let mut cwd = String::new();
            match env::current_exe() {
                Ok(path) => cwd = path.parent().unwrap().display().to_string(),
                Err(_) => cwd = "Not found".to_string(),
            };
            let filename = format!("{cwd}/config.ini");
            let config: Ini = Ini::load_from_file(filename).unwrap();
            let browser_section: &Properties = config.section(Some("browser")).unwrap();
            let gaming_section: &Properties = config.section(Some("gaming")).unwrap();
            let programming_section: &Properties = config.section(Some("programming")).unwrap();
            let system_section: &Properties = config.section(Some("system")).unwrap();
            let browsers: HashMap<String, String> = load_browser_section(browser_section);
            let gaming: HashMap<String, String> = load_gaming_section(gaming_section);
            let programming: HashMap<String, String> =
                load_programming_section(programming_section);
            let systems: HashMap<String, String> = load_system_section(system_section);
            let desired_downloads: Vec<String> =
                get_desired_downloads(browsers, gaming, programming, systems);

            for download in desired_downloads {
                let variable = links::map_config_key_to_function_name(&download);
                let link = variable
                    .get_download_link()
                    .unwrap_or(String::from("Unable to get download link"));
                if link == "Unable to get download link" {
                    println!("Unable to get download link for {download}, continuing...");
                    continue;
                }

                println!("Downloading {download}, please wait...");

                if link.starts_with("DU_CLI") {
                    // Handles cases where downloads are done through DUCLI (Download Utility Command Line Interface)
                    let split_link: Vec<&str> = link.split(" ").collect();
                    if split_link.len() == 3 {
                        let app_name = split_link[2];

                        cmd::run_cmd("download", app_name);
                    }
                } else {
                    // Handles the major of cases where a direct download link is supplied or scraped.
                    let result_object = download_file(&link);
                    match result_object {
                        Ok(()) => println!("{:?}", result_object.unwrap()),
                        Err(e) => println!("{:?}", e),
                    }
                    // println!("{:?}", result.unwrap());
                }
            }
        }
        ValidStarterArg::HELP => {
            let help_string = "Usage: wsb.exe <command> [arguments]
        Commands:
            download - Download something from somewhere.
            install - Install the thing.
            version - Print the current version.
            help - Show this message.

        Arguments:
            
            Press any key to exit...";

            println!("{}", help_string);
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).expect("");
            exit(0)
        }
        ValidStarterArg::INSTALL => {
            println!("Works")
        }
        ValidStarterArg::VERSION => {
            println!("WSB v0.0.1")
        }
    }
}

pub fn create_starter_args() -> Vec<String> {
    vec![
        String::from("download"),
        String::from("help"),
        String::from("install"),
        String::from("version"),
    ]
}

pub fn get_arg() -> String {
    let starter_args: Vec<String> = create_starter_args();
    //println!("{:?}", starter_args);
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("{}", "Invalid argument provided, please see below.");
    }

    let def: String = String::from("help");
    let raw_arg: &String = args.iter().nth(1).unwrap_or(&def);
    let arg_str: &mut String = &mut String::from(raw_arg); // Rust analyzer was screaming at me for not adding &mut
    let _: bool = is_valid_starter_arg(starter_args, arg_str); // Modifies the value of raw_arg

    // rename _ to result when debugging
    // println!("{}", result); // Used for debugging
    // println!("{:?}", arg_str); // Used for debugging
    return arg_str.to_string();
}

// starter_arg is mutable, as its modified to 'help' in the event an incorrect parameter is passed.
pub fn is_valid_starter_arg(starter_args: Vec<String>, starter_arg: &mut String) -> bool {
    let result: bool = starter_args.contains(&starter_arg);
    match result {
        true => return true,
        false => {
            *starter_arg = String::from("help"); // The * operator allows for a mutable param to be modified aslong as its still in scope.
            return false;
        }
    }
}

pub fn get_desired_downloads(
    browsers: HashMap<String, String>,
    gaming: HashMap<String, String>,
    programming: HashMap<String, String>,
    systems: HashMap<String, String>,
) -> Vec<String> {
    let mut desired_downloads: Vec<String> = Vec::new();

    for section in [browsers, gaming, programming, systems] {
        for program in section {
            if program.1 == "true" {
                desired_downloads.push(program.0);
            }
        }
    }
    desired_downloads
}

fn load_browser_section(browser_section: &Properties) -> HashMap<String, String> {
    let browser_names = vec![
        "Brave",
        "Chrome",
        "Chromium",
        "Edge",
        "Firefox",
        "Librefox",
        "Opera",
        "OperaGX",
        "PaleMoon",
        "Seamonkey-Experimental",
        "Tor",
        "Vivaldi-Experimental",
        "Waterfox",
    ];

    let mut browsers: HashMap<String, String> = HashMap::new();
    for browser_name in browser_names {
        browsers.insert(
            String::from(browser_name),
            browser_section
                .get(browser_name)
                .unwrap_or("false")
                .to_string(),
        );
    }
    // Used for debugging
    // for browser in browsers {
    //     println!("{:?}", browser);
    // }
    browsers
}

fn load_gaming_section(gaming_section: &Properties) -> HashMap<String, String> {
    let app_names: Vec<&str> = vec![
        "AMDAutoDetect",
        "BattleNetLauncher",
        "BluestacksEmulator",
        "CPUZ",
        "CurseForge",
        "EpicGamesLauncher",
        "GPUZ",
        "HWiNFO",
        "HWMonitor",
        "ItchIoLauncher",
        "MSIAfterburner",
        "NexusManager",
        "NvidiaApp",
        "OBSStudio",
        "Parsec",
        "PingPlotter",
        "ProcessLasso",
        "RazerCortex",
        "Reshade",
        "RockstarLauncher",
        "RobloxLauncher",
        "StreamlabsOBS",
        "SteamLauncher",
        "WTFast",
    ];

    let mut apps: HashMap<String, String> = HashMap::new();

    for app_name in app_names {
        apps.insert(
            String::from(app_name),
            gaming_section.get(app_name).unwrap_or("false").to_string(),
        );
    }

    // Used for debugging
    // for app in apps {
    //     println!("{:?}", app);
    // }

    apps
}

fn load_programming_section(programming_section: &Properties) -> HashMap<String, String> {
    let app_names = vec![
        "AndroidStudio",
        "AzureDataStudio",
        "BurpSuite",
        "DockerDesktop",
        "FileZillaClient",
        "FileZillaServer",
        "GithubDesktop",
        "EclipseIDE",
        "FiddlerClassic",
        "FiddlerEverywhere",
        "Kubernetes",
        "MongoDB",
        "MySQLWorkbench",
        "Nmap",
        "NodeJS",
        "NotepadPlusPlus",
        "Npcap",
        "Ollama",
        "OracleVirtualBox",
        "OracleVirtualBoxExtPack",
        "OWASPZAP",
        "PostgreSQL",
        "Postman",
        "Python2_7_18",
        "Python3_8_10",
        "Python3_9_10",
        "Python3_10_10",
        "Python3_11_10",
        "Python3_12_9",
        "Python3_13_2",
        "PuTTY",
        "SublimeText",
        "UTM",
        "VisualStudio",
        "VisualStudioCode",
        "VSCodium",
        "WebStorm",
        "WinGet",
        "WireShark",
        "XAMPP",
    ];

    let mut apps: HashMap<String, String> = HashMap::new();

    for app_name in app_names {
        apps.insert(
            String::from(app_name),
            programming_section
                .get(app_name)
                .unwrap_or("false")
                .to_string(),
        );
    }

    // Used for debugging
    // for app in apps {
    //     println!("{:?}", app);
    // }

    apps
}

fn load_system_section(system_section: &Properties) -> HashMap<String, String> {
    let app_names = vec![
        "7Zip",
        "Audacious",
        "LibreOffice",
        "ModernCSV",
        "NoMacs",
        "Okular",
        "Rufus",
        "VLC",
        "WinRAR",
    ];

    let mut apps: HashMap<String, String> = HashMap::new();

    for app_name in app_names {
        apps.insert(
            String::from(app_name),
            system_section.get(app_name).unwrap_or("false").to_string(),
        );
    }

    // Used for debugging
    /*for  (key, value) in &system_tools {
       printlnprintln_string_err("{}: {}", key, value);
    }*/

    apps
}

fn decode_url(encoded_url: &str) -> String {
    let owned_decoded_object = decode(encoded_url).expect("UTF-8").to_owned();
    let decoded_filename = owned_decoded_object.to_string();
    let filename_result = decoded_filename.clone();
    return filename_result;
}

#[allow(unused_variables)]
#[allow(unused_assignments)] // although cwd is used the fact its overwritten before its initialized requires this flag
fn download_file(download_link: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (status, response_bytes, headers) = make_web_request_w_ua(download_link)?; // ? Handles cases where an Err is raised, returning Ok() or the Err()

    match status {
        StatusCode::OK => {
            let mut cwd = String::new();
            match env::current_exe() {
                Ok(path) => cwd = path.parent().unwrap().display().to_string(),
                Err(error) => cwd = error.to_string(),
            };
            let content_disposition = headers.get("content-disposition");
            let content_encoding = headers.get("content-encoding");
            let final_url = headers.get("location");

            if content_disposition.is_none()
                && final_url.is_none()
                && !download_link.ends_with(".zip")
            {
                println!("Unable to download file");
                return Err("Unable to download file".into());
            }

            let content_disposition_value = content_disposition
                .map(|value| {
                    value.to_str().unwrap_or_else(|_| {
                        print!("Unable to download.. A");
                        return "Not found";
                    })
                })
                .unwrap_or_else(|| {
                    // Content disposition's value was unable to be unwrapped
                    return "Not found";
                });

            let content_encoding_value = content_encoding
                .map(|value| {
                    value.to_str().unwrap_or_else(|_| {
                        println!("Unable to download.. B");
                        return "Not found";
                    })
                })
                .unwrap_or_else(|| {
                    // Final url's value was unable to be unwrapped
                    return "Not found";
                });

            let final_url_value = final_url
                .map(|value| {
                    value.to_str().unwrap_or_else(|_| {
                        println!("Unable to download.. C");
                        return "Not found";
                    })
                })
                .unwrap_or_else(|| {
                    // Final url's value was unable to be unwrapped
                    return "Not found";
                });

            if content_disposition_value == "Not Found" && final_url_value == "Not Found" {
                if !download_link.ends_with(".zip") || !download_link.contains(".exe?installer_id=")
                {
                    println!("Unable to parse filename, continuing..");
                    return Err("Unable to parse filename, continuing..".into());
                }
            }

            let mut parsed_url_value = final_url_value;
            if parsed_url_value.starts_with("/http") {
                parsed_url_value = &parsed_url_value[1..]; // removes the leading / from the url value
            }
            let mut file = Url::parse(parsed_url_value).unwrap();
            let mut filename: &str = "";
            let mut decoded_filename: String = String::new();
            let mut found = false;
            while !found {
                if final_url_value.contains(".exe?") {
                    file.set_query(None);
                    let mut raw_filename = file.as_str().split("/").last().unwrap();
                    if raw_filename.contains("?filename=") {
                        raw_filename = raw_filename.split("?filename=").next().unwrap();
                    }
                    //println!("{}", raw_filename);
                    filename = raw_filename;
                    found = true;
                } else if final_url_value.contains("&installer=Battle.net") {
                    filename = "battle-net-setup.exe";
                    found = true;
                } else if content_disposition_value.contains("filename=") && !found {
                    filename = content_disposition_value
                        .split("filename=")
                        .last()
                        .unwrap_or_else(|| return "Not found");

                    found = true;
                } else if final_url_value.ends_with(".exe") && !found
                    || final_url_value.ends_with(".msi") && !found
                    || final_url_value.ends_with(".zip") && !found
                {
                    let encoded_filename = final_url_value
                        .split("/")
                        .last()
                        .unwrap_or_else(|| return "Not found");
                    decoded_filename = decode_url(encoded_filename);
                    filename = &decoded_filename;
                    found = true;
                } else if download_link.ends_with(".exe") && !found
                    || download_link.ends_with(".msi") && !found
                    || download_link.ends_with(".zip") && !found
                {
                    filename = download_link
                        .split("/")
                        .last()
                        .unwrap_or_else(|| return "Not found");

                    found = true;
                } else if download_link.contains(".exe?installer_id=") && !found {
                    filename = download_link.split("?installer_id=").next().unwrap();
                    found = true;
                } else {
                    filename = "Not found";
                    if !found {
                        println!("Unable to parse filename from response, continuing...");
                        return Err("Not found".into());
                    }
                }
            }

            println!("\nDownloading: {}", filename);

            if filename == "Not found" {
                return Err("Unable to parse filename from response, continuing...".into());
            }

            match fs::create_dir(format!("{cwd}/applications")) {
                Ok(()) => println!("Applications directory created, continuing..."),
                Err(_) => println!("Applications directory already exists, continuing..."),
            };
            let mut file: Result<File, std::io::Error> =
                File::create(format!("{cwd}/applications/{filename}"));

            match &mut file {
                Ok(file_obj) => {
                    // Used for debugging response
                    // for (key, value) in headers.iter() {
                    //     println!("{}: {:?}", key, value);
                    // }
                    let _ = file_obj.write(&response_bytes);
                    println!("Finished downloaded for: {filename}");
                }
                Err(e) => {
                    return Err(format!(
                        "Failed to parse response..\nrequest status: {}\nError:\n{}",
                        status, e
                    )
                    .into());
                }
            }
            Ok(())
        }
        _ => {
            if status.as_u16() == 410 {
                println!("An error occured while downloading.. continuing..");
                return Ok(());
            }
            println!("Failed to parse response");
            return Err("Failed to parse response.".into());
        }
    }
}
