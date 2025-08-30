use crate::browser::Browser;
use crate::gaming::Gaming;
use crate::programming::{FilezillaType, Programming};
use crate::system::Utility;

use reqwest::StatusCode;
use reqwest::blocking::{Client, Response, get};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use scraper::{Html, Selector};
use serde_json::{Value, from_value};
use std::error::Error;
use std::process::exit;
use tl::{ParserOptions, parse};
use xmltojson::to_json;

#[allow(dead_code)]
#[derive(Debug)]
pub enum KeyType {
    Browser(Browser),
    Gaming(Gaming),
    Programming(Programming),
    Utility(Utility),
}

impl KeyType {
    pub fn get_download_link(&self) -> Option<String> {
        match self {
            KeyType::Browser(b) => b.get_link(),
            KeyType::Gaming(g) => g.get_link(),
            KeyType::Programming(p) => p.get_link(),
            KeyType::Utility(u) => u.get_link(),
        }
    }
}

fn make_web_request(url: &str) -> Result<(StatusCode, String), Box<dyn Error>> {
    let response: Response = get(url)?;
    let status: StatusCode = response.status();
    if status.is_success() {
        let content: String = response.text()?;
        Ok((status, content))
    } else {
        let status: StatusCode = response.status();
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Request failed with status: {}", status),
        )))
    }
}

pub fn make_web_request_v2(url: &str) -> Result<(StatusCode, String, HeaderMap), Box<dyn Error>> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"));

    let response = client.get(url).headers(headers).send()?;
    let status = response.status();
    let resp_headers = response.headers().clone();
    if status.is_success() {
        let body = response.text()?;
        Ok((status, body, resp_headers))
    } else {
        let status: StatusCode = response.status();
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Request failed with status {}, please try again", status),
        )))
    }
}

#[allow(dead_code)]
pub fn map_config_key_to_function_name(config_key: &str) -> KeyType {
    match config_key {
        "Brave" => KeyType::Browser(Browser::Brave),
        "Chrome" => KeyType::Browser(Browser::Chrome),
        "Chromium" => KeyType::Browser(Browser::Chromium),
        "Edge" => KeyType::Browser(Browser::Edge),
        "Firefox" => KeyType::Browser(Browser::Firefox),
        "Librewolf" => KeyType::Browser(Browser::Librewolf),
        "Opera" => KeyType::Browser(Browser::Opera),
        "OperaGX" => KeyType::Browser(Browser::OperaGX),
        "PaleMoon" => KeyType::Browser(Browser::PaleMoon),
        "Seamonkey-Experimental" => KeyType::Browser(Browser::SeaMonkey),
        "Tor" => KeyType::Browser(Browser::Tor),
        "Vivaldi-Experimental" => KeyType::Browser(Browser::Vivaldi),
        "Waterfox" => KeyType::Browser(Browser::Waterfox),
        "AMDAutoDetect" => KeyType::Gaming(Gaming::AMDAutoDetect),
        "BattleNetLauncher" => KeyType::Gaming(Gaming::BattleNetLauncher),
        "BluestacksEmulator" => KeyType::Gaming(Gaming::BluestacksEmulator),
        "CPUZ" => KeyType::Gaming(Gaming::CPUZ),
        "CurseForge" => KeyType::Gaming(Gaming::CurseForge),
        "EpicGamesLauncher" => KeyType::Gaming(Gaming::EpicGamesLauncher),
        "GPUZ" => KeyType::Gaming(Gaming::GPUZ),
        "HWiNFO" => KeyType::Gaming(Gaming::HWiNFO),
        "HWMonitor" => KeyType::Gaming(Gaming::HWMonitor),
        "MSIAfterburner" => KeyType::Gaming(Gaming::MSIAfterburner),
        "NvidiaApp" => KeyType::Gaming(Gaming::NvidiaApp),
        "OBSStudio" => KeyType::Gaming(Gaming::OBSStudio),
        "Parsec" => KeyType::Gaming(Gaming::Parsec),
        "PingPlotter" => KeyType::Gaming(Gaming::PingPlotter),
        "ProcessLasso" => KeyType::Gaming(Gaming::ProcessLasso),
        "RazerCortex" => KeyType::Gaming(Gaming::RazerCortex),
        "Reshade" => KeyType::Gaming(Gaming::Reshade),
        "RockstarLauncher" => KeyType::Gaming(Gaming::RockstarLauncher),
        "RobloxLauncher" => KeyType::Gaming(Gaming::RobloxLauncher),
        "StreamlabsOBS" => KeyType::Gaming(Gaming::StreamlabsOBS),
        "SteamLauncher" => KeyType::Gaming(Gaming::SteamLauncher),
        "WTFast" => KeyType::Gaming(Gaming::WTFast),
        "AndroidStudio" => KeyType::Programming(Programming::AndroidStudio),
        "AzureDataStudio" => KeyType::Programming(Programming::AzureDataStudio),
        "BurpSuite" => KeyType::Programming(Programming::BurpSuite),
        "DockerDesktop" => KeyType::Programming(Programming::DockerDesktop),
        "FileZillaClient" => KeyType::Programming(Programming::FileZilla(FilezillaType::Client)),
        "FileZillaServer" => KeyType::Programming(Programming::FileZilla(FilezillaType::Server)),
        "GithubDesktop" => KeyType::Programming(Programming::GithubDesktop),
        "EclipseIDE" => KeyType::Programming(Programming::EclipseIDE),
        "FiddlerClassic" => KeyType::Programming(Programming::FiddlerClassic),
        "FiddlerEverywhere" => KeyType::Programming(Programming::FiddlerEverywhere),
        "Kubernetes" => KeyType::Programming(Programming::Kubernetes),
        "MongoDB" => KeyType::Programming(Programming::MongoDB),
        "MySQLWorkbench" => KeyType::Programming(Programming::MySQLWorkbench),
        "Nmap" => KeyType::Programming(Programming::Nmap),
        "NodeJS" => KeyType::Programming(Programming::NodeJS),
        "NotepadPlusPlus" => KeyType::Programming(Programming::NotepadPlusPlus),
        "Npcap" => KeyType::Programming(Programming::Npcap),
        "Ollama" => KeyType::Programming(Programming::Ollama),
        "OracleVirtualBox" => KeyType::Programming(Programming::OracleVirtualBox),
        "OracleVirtualBoxExtPack" => KeyType::Programming(Programming::OracleVirtualBoxExtPack),
        "OWASPZAP" => KeyType::Programming(Programming::OWASPZAP),
        "PostgreSQL" => KeyType::Programming(Programming::PostgreSQL),
        "Postman" => KeyType::Programming(Programming::Postman),
        "Python2_7_18" => KeyType::Programming(Programming::Python2_7_18),
        "Python3_8_10" => KeyType::Programming(Programming::Python3_8_10),
        "Python3_9_10" => KeyType::Programming(Programming::Python3_9_10),
        "Python3_10_10" => KeyType::Programming(Programming::Python3_10_10),
        "Python3_11_10" => KeyType::Programming(Programming::Python3_11_10),
        "Python3_12_9" => KeyType::Programming(Programming::Python3_12_9),
        "Python3_13_2" => KeyType::Programming(Programming::Python3_13_2),
        "PuTTY" => KeyType::Programming(Programming::PuTTY),
        "SublimeText" => KeyType::Programming(Programming::SublimeText),
        "UTM" => KeyType::Programming(Programming::UTM),
        "VisualStudio" => KeyType::Programming(Programming::VisualStudio),
        "VisualStudioCode" => KeyType::Programming(Programming::VisualStudioCode),
        "VSCodium" => KeyType::Programming(Programming::VSCodium),
        "WebStorm" => KeyType::Programming(Programming::WebStorm),
        "WinGet" => KeyType::Programming(Programming::WinGet),
        "WireShark" => KeyType::Programming(Programming::WireShark),
        "XAMPP" => KeyType::Programming(Programming::XAMPP),
        "7Zip" => KeyType::Utility(Utility::SevenZip),
        "Audacious" => KeyType::Utility(Utility::Audacious),
        "LibreOffice" => KeyType::Utility(Utility::LibreOffice),
        "ModernCSV" => KeyType::Utility(Utility::ModernCSV),
        "NoMacs" => KeyType::Utility(Utility::NoMacs),
        "Okular" => KeyType::Utility(Utility::Okular),
        "Rufus" => KeyType::Utility(Utility::Rufus),
        "VLC" => KeyType::Utility(Utility::VLC),
        "WinRAR" => KeyType::Utility(Utility::WinRAR),
        _ => {
            println!("Unable to find app with the entered name.");
            exit(0);
        }
    }
}

// region: Browser Download Functions
pub fn get_brave_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://github.com/brave/brave-browser/releases/latest").unwrap();
    let raw_html = result.1;
    if result.0 != StatusCode::OK {
        return String::from("No download link found");
    } else {
        let html: Html = Html::parse_document(&raw_html);
        let selector: Selector = Selector::parse("span.ml-1").unwrap();
        if let Some(raw_version) = html
            .select(&selector)
            .filter_map(|href| href.text().next())
            .next()
        {
            let version = raw_version.trim();
            let link = format!(
                "https://github.com/brave/brave-browser/releases/download/{version}/BraveBrowserSetup.exe"
            );
            //println!("{}", version);
            link
        } else {
            String::from("No download link found")
        }
    }
}

pub fn get_chrome_link() -> String {
    "https://dl.google.com/chrome/install/ChromeStandaloneSetup64.exe".to_string()
}

pub fn get_chromium_link() -> String {
    "https://download-chromium.appspot.com/dl/Win_x64?type=snapshots".to_string()
}

pub fn get_edge_link() -> String {
    "https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/f9137856-3c6a-4a53-8a62-6c3b539149b0/MicrosoftEdgeEnterpriseX64.msi".to_string()
}

pub fn get_opera_link() -> String {
    "https://net.geo.opera.com/opera/stable/windows".to_string()
}

pub fn get_operagx_link() -> String {
    "https://net.geo.opera.com/opera_gx/stable/windows".to_string()
}

pub fn get_firefox_link() -> String {
    "https://download.mozilla.org/?product=firefox-stub&os=win&lang=en-US".to_string()
}

pub fn get_librewolf_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://gitlab.com/librewolf-community/browser/bsys6/-/releases.atom")
            .unwrap();
    //let status: StatusCode = result.0; // Request status
    let html: String = result.1; // Request response (HTML)
    let converted_obj: Result<Value, xmltojson::Error> = to_json(&html);
    let val: Value = converted_obj.unwrap();
    let parsed_obj: Value = from_value(val).unwrap();
    let release_id: String = parsed_obj["feed"]["entry"][0]["id"].to_string();
    let problem_text: String = '"'.to_string();
    let new_text: String = String::new();
    let release_dir: &str = release_id.split("/").last().unwrap();
    let version: String = release_dir.replace(&problem_text, &new_text);
    let url: String = format!(
        "https://gitlab.com/api/v4/projects/44042130/packages/generic/librewolf/{version}/librewolf-{version}-windows-x86_64-package.zip"
    );

    //println!("{}", url);
    url
}

pub fn get_palemoon_link() -> String {
    "https://www.palemoon.org/download.php?mirror=us&bits=64&type=installer".to_string()
}

pub fn get_seamonkey_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://www.seamonkey-project.org/releases/").unwrap();
    //let status: StatusCode = result.0; // Request status
    let html: String = result.1; // Request response (HTML)
    let document: Html = Html::parse_document(&html);
    let selector: Selector = Selector::parse(".curVersion").unwrap();
    let version = document
        .select(&selector) // Returns an Iterator of all <a> Element objects from the parsed tree
        .filter_map(|element| element.text().next()) // Filters the element objects, returning the first element's text
        .map(|href: &str| href.to_string()) // Maps the hrefs to a String Iterator
        .next()
        .unwrap();
    format!(
        "https://archive.seamonkey-project.org/releases/{version}/win32/en-US/seamonkey-{version}.en-US.win32.installer.exe"
    )
}

pub fn get_tor_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://www.torproject.org/download/tor/").unwrap();
    //let status: StatusCode = result.0; // Request status
    let html: String = result.1; // Request response (HTML)
    let document: Html = Html::parse_document(&html);
    let selector: Selector = Selector::parse("a.downloadLink").unwrap();
    let temp_link = document
        .select(&selector)
        .filter_map(|element| {
            element
                .value()
                .attr("href")
                .filter(|href| href.contains("-x86_64-"))
        })
        .map(|href| href.to_string())
        .next()
        .unwrap();
    //println!("{}", temp_link);
    let version = temp_link.split("/").nth(5).unwrap();
    //println!("{}", version); //
    let url: String = format!(
        "https://www.torproject.org/dist/torbrowser/{version}/tor-browser-windows-x86_64-portable-{version}.exe"
    );
    url
}

pub fn _get_vivaldi_link_old() -> String {
    let result: (StatusCode, String) = make_web_request("https://vivaldi.com/download/").unwrap();
    //let status: StatusCode = result.0; // Request status
    let html: String = result.1; // Request response (HTML)
    let document: Html = Html::parse_document(&html); // Parses the provided html string into an Html tree object
    let selector: Selector = Selector::parse("a").unwrap(); // Creates a Selector object to parse all <a> Elements
    document
        .select(&selector) // Returns an Iterator of all <a> Element objects from the parsed tree
        .filter_map(|element| {
            element
                .value()
                .attr("href")
                .filter(|href| href.contains(".exe"))
                .filter(|href| href.contains("x64"))
        }) // Filters the element objects, returning only href links containing ".exe" and the defined PLATFORM "amd64"
        .map(|href: &str| href.to_string()) // Maps the hrefs to a String Iterator
        .next()
        .unwrap()
    // Returns the String Iterator
}

pub fn get_vivaldi_link() -> String {
    String::from("DU_CLI.exe download Vivaldi")
}

pub fn get_waterfox_link() -> String {
    let result: (StatusCode, String) = make_web_request("https://waterfox.net/download/").unwrap();
    //let status: StatusCode = result.0; // Request status, used for debugging
    let raw_html: String = result.1; // Request response (HTML)
    //println!("{}", raw_html); // Used for debugging
    let html: Html = Html::parse_document(&raw_html);
    let selector: Selector = Selector::parse("#release-version").unwrap();
    let version: String = html
        .select(&selector)
        .flat_map(|element| element.text())
        .collect();
    let link: String = format!(
        "https://cdn1.waterfox.net/waterfox/releases/{version}/WINNT_x86_64/Waterfox%20Setup%20{version}.exe"
    );
    link
}
// endregion: Browser Download Functions

// region: Gaming Download Functions

//Refactor this function to scrape the newest version dynamically via https://www.amd.com/en/support/download/drivers.html
pub fn _get_amd_autodetect_link_old() -> String {
    "https://drivers.amd.com/drivers/installer/24.30/whql/amd-software-adrenalin-edition-25.3.1-minimalsetup-250312_web.exe".to_string()
}

pub fn get_amd_autodetect_link() -> String {
    String::from("DU_CLI.exe download AMD")
}

pub fn get_battlenet_link() -> String {
    String::from(
        "https://downloader.battle.net/download/getInstaller?os=win&installer=Battle.net-Setup.exe",
    )
}

pub fn get_bluestacks_link() -> String {
    String::from(
        "https://cloud.bluestacks.com/api/getdownloadnow?platform=win&win_version=10&bluestacks_version=bs5",
    )
}

// Refactor this function to handle cases for the custom installations
pub fn get_cpuz_link() -> String {
    String::from("https://www.cpuid.com/downloads/cpu-z/cpu-z_2.15-en.exe")
}

pub fn get_curseforge_link() -> String {
    String::from(
        "https://download.overwolf.com/install/Download?ExtensionId=cfiahnpaolfnlgaihhmobmnjdafknjnjdpdabpcm",
    )
}

pub fn get_epic_games_link() -> String {
    String::from(
        "https://launcher-public-service-prod06.ol.epicgames.com/launcher/api/installer/download/EpicGamesLauncherInstaller.msi",
    )
}

// Refactor this to dynamically download the latest version and offer the ROG themed alternative
pub fn get_gpuz_link() -> String {
    String::from(
        "https://us9-dl.techpowerup.com/files/kv4QONlJeD527EtQMrtjYQ/1743512769/GPU-Z.2.64.0.exe",
    )
}

// Refactor the dynamically download the latest version using the selector div.col-md-6:nth-child(1) > div:nth-child(1) > sub:nth-child(3)
pub fn get_hwinfo_link() -> String {
    String::from("https://www.hwinfo.com/files/hwi64_822.exe")
}

pub fn get_hwmonitor_link() -> String {
    String::from("https://www.cpuid.com/downloads/hwmonitor/hwmonitor_1.56.exe")
}

pub fn get_msi_afterburner_link() -> String {
    String::from(
        "https://www.guru3d.com/getdownload/2c1b2414f56a6594ffef91236a87c0e976d52e0518b43f3846bab016c2f20c7c4d6ce7dfe1991cc241d59b5c8cb07e5018b083a5902ac6c67fbe3b852ca022b0f73541638028a2d270eb576309b5208d7642bced763e8806fd9c5a9bca00d71e03e3f895d9924372aebbd01f8d3b8f4f270059bd6d5516b53f1cebbb3340fa764f68932d48b5bb538878337e2e92244ec842c6bc8fbe77fb2097b27ac094473cbbffdfdca7be83b46c55febb094e360b65a50d97cc2f5ebe7b2f727003a739d719662666b53ff47a62585c6739",
    )
}

pub fn get_nvda_app_link() -> String {
    String::from(
        "https://us.download.nvidia.com/nvapp/client/11.0.3.213/NVIDIA_app_beta_v11.0.3.213.exe",
    )
}

pub fn get_obs_studio_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://github.com/obsproject/obs-studio/releases/latest").unwrap();
    let raw_html = result.1;
    if result.0 != StatusCode::OK {
        return String::from("No download link found");
    } else {
        let html: Html = Html::parse_document(&raw_html);
        let selector: Selector = Selector::parse("span.ml-1").unwrap();
        if let Some(raw_version) = html
            .select(&selector)
            .filter_map(|href| href.text().next())
            .next()
        {
            let version = raw_version.trim();
            let link = format!(
                "https://github.com/obsproject/obs-studio/releases/download/{version}/OBS-Studio-{version}-Windows-Installer.exe"
            );
            //println!("{}", version);
            link
        } else {
            String::from("No download link found")
        }
    }
}

pub fn get_parsec_link() -> String {
    String::from("https://builds.parsec.app/package/parsec-windows.exe")
}

pub fn get_pingplotter_link() -> String {
    String::from("https://www.pingplotter.com/downloads/pingplotter_install.exe")
}

pub fn get_process_lasso_link() -> String {
    String::from("https://dl.bitsum.com/files/processlassosetup64.exe")
}

pub fn get_razer_cortex_link() -> String {
    String::from("https://dl.razerzone.com/drivers/GameBooster/RazerCortexInstaller.exe")
}

pub fn get_reshade_link() -> String {
    String::from("https://reshade.me/downloads/ReShade_Setup_6.4.1.exe")
}

pub fn get_rockstar_launcher_link() -> String {
    String::from(
        "https://gamedownloads.rockstargames.com/public/installer/Rockstar-Games-Launcher.exe",
    )
}

pub fn _get_roblox_launcher_link_old() -> String {
    // let result = make_web_request_v2("https://www.roblox.com/download/client?os=win").unwrap();
    // let header_value = result.2;
    // for (key, value) in header_value {
    //     print!("{:?}", key.unwrap().as_str());
    //     print!("{:?}", value.to_str());
    //     print!("\n");
    // }
    String::from("https://www.roblox.com/download/client?os=win")
}

pub fn get_roblox_launcher_link() -> String {
    String::from("DU_CLI.exe download Roblox")
}
pub fn get_streamlabs_obs_link() -> String {
    String::from("https://streamlabs.com/streamlabs-desktop/download")
}

pub fn get_steam_link() -> String {
    String::from("https://cdn.fastly.steamstatic.com/client/installer/SteamSetup.exe")
}

pub fn get_wtfast_link() -> String {
    String::from("https://download.wtfast.com/product/wtfast/")
}

// endregion: Gaming Download Functions

// region: Programming Download Functions

pub fn get_android_studio_link() -> String {
    String::from(
        "https://redirector.gvt1.com/edgedl/android/studio/install/2024.3.1.14/android-studio-2024.3.1.14-windows.exe",
    )
}

pub fn get_azure_link() -> String {
    String::from("https://go.microsoft.com/fwlink/?linkid=2302007")
}

pub fn get_burp_suite_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://portswigger.net/burp/releases").unwrap();
    // let status: StatusCode = result.0; // Request status
    // println!("{}", status);
    let html: String = result.1; // Request response (HTML)
    let dom = parse(&html, ParserOptions::default()).unwrap(); // Uses tl to parse the html into dom
    let parser = dom.parser(); // Initalizes a parser object from the dom
    let mut link: String = String::new();
    let mut links_iterated = 0;

    // Iterates through the div tag and if it's found continues, if not returns "No class found"
    if let Some(noscript) = dom.nodes().iter().find(|node| {
        node.as_tag()
            .and_then(|tag| tag.attributes().get("class"))
            .map(|class| {
                class.as_ref().expect("No class found").as_utf8_str() == "noscript-postlist"
            })
            .unwrap_or(false)
    }) {
        // Iterates through all children of the div to find the second a tag with an href that contains "professional-community-"
        // The first tag contains a beta release, and the second tag contains the most recent stable release.
        if let Some(children) = noscript.as_tag().map(|tag| tag.children()) {
            for element in children.all(parser) {
                if let Some(a_tag) = element.as_tag() {
                    if let Some(href) = a_tag.attributes().get("href") {
                        links_iterated += 1;

                        // Continue checking until the second valid link is iterated
                        if href.is_some() && links_iterated < 2 {
                            link = String::from(href.as_ref().unwrap().as_utf8_str());
                        } else if href.is_some() && links_iterated == 2 {
                            link = String::from(href.as_ref().unwrap().as_utf8_str());
                            break;
                        } else {
                            link = String::from("No link found");
                        }

                        // println!(
                        //     "Found link: {}",
                        //     href.as_ref().expect("No href found").as_utf8_str()
                        // );
                    }
                }
            }
        }
    } else {
        println!("No <div class='noscript-postlist'> found.");
        link = String::from("No link found");
    }
    link

    // let document = Html::parse_document(&html);

    // // Selector for the a tag containing the href link being parsed
    // let selector = Selector::parse("//a[contains(text(),'Professional / Community']").unwrap();

    // let version = document
    //     .select(&selector)
    //     .filter_map(|element| element.value().attr("href"))
    //     .map(|href| format!("https://portswigger.net{}", href))
    //     .nth(0)
    //     .unwrap();

    // version
    // Returns the String Iterator
}

pub fn get_docker_desktop_link() -> String {
    String::from("https://desktop.docker.com/win/main/amd64/Docker%20Desktop%20Installer.exe")
}

pub fn get_filezilla_link(filezilla_type: FilezillaType) -> String {
    match filezilla_type {
        FilezillaType::Client => String::from(
            "https://download.filezilla-project.org/client/FileZilla_3.68.1_win64_sponsored2-setup.exe",
        ),
        FilezillaType::Server => String::from(
            "https://dl3.cdn.filezilla-project.org/server/FileZilla_Server_1.9.4_win64-setup.exe?h=Tkmnw8TQh-e_FicKR3lm4Q&x=1742954205",
        ),
    }
}

pub fn get_github_desktop_link() -> String {
    String::from("https://central.github.com/deployments/desktop/desktop/latest/win32")
}

pub fn get_eclipse_ide_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://www.eclipse.org/downloads/packages/").unwrap();
    //let status: StatusCode = result.0; // Request status, used for debugging
    let raw_html: String = result.1; // Request response (HTML)
    //println!("{}", raw_html); // Used for debugging
    let html: Html = Html::parse_document(&raw_html);
    let selector: Selector = Selector::parse("a[title='x86_64 Download']").unwrap();
    let link: Option<String> = html
        .select(&selector)
        .filter_map(|element| {
            element
                .value()
                .attr("href")
                .filter(|element| element.contains(".exe"))
                .map(|element| String::from(element))
        })
        .next();
    // println!("{:?}", link); // used for debugging
    link.unwrap_or_else(|| String::from("No download link found")) // Not sure what the || operator does but rust-analyzer required it, will research later.
}

pub fn get_fiddler_classic_link() -> String {
    String::from(
        "https://downloads.getfiddler.com/fiddler-classic/FiddlerSetup.5.0.20251.1171-latest.exe",
    ) // No longer updated as compared to fiddler everywhere
}

pub fn get_fiddler_everywhere_link() -> String {
    String::from("https://downloads.getfiddler.com/win/Fiddler%20Everywhere%206.3.0.exe")
}

pub fn get_kubernetes_link() -> String {
    let result: Result<(StatusCode, String), Box<dyn Error>> =
        make_web_request("https://dl.k8s.io/release/stable.txt");
    //let status: StatusCode = result.0; // Request status, used for debugging
    let version: String = match result {
        Ok((_, html)) => html, // Extract HTML if request is successful
        Err(_) => return String::from("No download link found"), // Return error message if request fails
    };
    String::from(format!(
        "https://dl.k8s.io/release/{version}/bin/windows/amd64/kubectl.exe"
    ))
}

pub fn get_mongodb_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://www.mongodb.com/try/download/compass").unwrap();
    let raw_html: String = result.1; // Request response (HTML)

    let html: Html = Html::parse_document(&raw_html);
    let selector = Selector::parse(r#"script[id="server-data"]"#).unwrap();

    if let Some(element) = html.select(&selector).next() {
        // If the selector is present
        if let Some(script_text) = element.text().next() {
            let json_str = script_text
                .trim()
                .strip_prefix("window.__serverData=")
                .unwrap_or(script_text); // Removes the JavaScript code to return raw JSON

            let json: serde_json::Value = match serde_json::from_str(json_str) {
                Ok(value) => value,
                Err(_) => return String::from("No download link found."),
            };

            // Parses the msi installer link
            if let Some(link) = json["components"]
                .get(2)
                .and_then(|element| element.get("props"))
                .and_then(|element| element.get("embeddedComponents"))
                .and_then(|element| element.get(0))
                .and_then(|element| element.get("props"))
                .and_then(|element| element.get("items"))
                .and_then(|element| element.get(3))
                .and_then(|element| element.get("embeddedComponents"))
                .and_then(|element| element.get(0))
                .and_then(|element| element.get("props"))
                .and_then(|element| element.get("data"))
                .and_then(|element| element.get(0))
                .and_then(|element| element.get("data"))
                .and_then(|element| element.get(0))
                .and_then(|element| element.get("2.4.2"))
                .and_then(|element| element.get("platforms"))
                .and_then(|element| element.get("Windows x64 (10+)"))
                .and_then(|element| element.get("msi"))
                .and_then(|element| element.as_str())
            {
                return link.to_string();
            }
        }
        return String::from("No download link found.");
    }
    return String::from("No download link found.");
}

pub fn get_mysql_workbench_link() -> String {
    String::from("https://dev.mysql.com/downloads/file/?id=536668")
}

pub fn get_nmap_link() -> String {
    let result = make_web_request("https://nmap.org/download.html#windows").unwrap();
    let raw_html: String = result.1;
    let html: Html = Html::parse_document(&raw_html);
    let selector = Selector::parse("a").unwrap();
    let elements: Vec<&str> = html
        .select(&selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            if href.contains("https://nmap.org/dist/nmap-") && href.contains(".exe") {
                Some(href)
            } else {
                None
            }
        })
        .collect();

    if let Some(link) = elements.first() {
        link.to_string()
    } else {
        String::from("No download link found")
    }
}

pub fn get_nodejs_link() -> String {
    String::from("https://nodejs.org/download/release/latest/win-x64/node.exe")
}

pub fn get_notepadplusplus_link() -> String {
    let result = make_web_request("https://notepad-plus-plus.org/downloads/").unwrap();
    let raw_html: String = result.1;
    let html: Html = Html::parse_document(&raw_html);
    let selector =
        Selector::parse(r#"a[href*="https://notepad-plus-plus.org/downloads/"]"#).unwrap();
    let elements: Vec<String> = html
        .select(&selector)
        .filter_map(|href| href.value().attr("href").map(|href| href.to_string()))
        .collect();

    if let Some(link) = elements.first() {
        link.to_string()
    } else {
        String::from("No download link found")
    }

    //.patterns-list > li:nth-child(1) > h2:nth-child(1) > a:nth-child(1)
}

pub fn get_npcap_link() -> String {
    let result = make_web_request("https://nmap.org/download.html#windows").unwrap();
    let raw_html: String = result.1;
    let html: Html = Html::parse_document(&raw_html);
    let selector = Selector::parse("a").unwrap();
    let elements: Vec<&str> = html
        .select(&selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            if href.contains("https://npcap.com/dist/npcap-") && href.contains(".exe") {
                Some(href)
            } else {
                None
            }
        })
        .collect();

    if let Some(link) = elements.first() {
        link.to_string()
    } else {
        String::from("No download link found")
    }
}

pub fn get_ollama_link() -> String {
    String::from("https://ollama.com/download/OllamaSetup.exe")
}

pub fn get_oracle_virtualbox_link() -> String {
    let result = make_web_request("https://www.virtualbox.org/wiki/Downloads").unwrap();
    let raw_html: String = result.1;
    let html: Html = Html::parse_document(&raw_html);
    let selector = Selector::parse(".ext-link").unwrap();

    let elements: Vec<&str> = html
        .select(&selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            if href.contains(".exe") {
                Some(href)
            } else {
                None
            }
        })
        .collect();

    if let Some(link) = elements.first() {
        link.to_string()
    } else {
        String::from("No download link found")
    }
}

pub fn get_oracle_vbox_ext_pack_link() -> String {
    let result = make_web_request("https://www.virtualbox.org/wiki/Downloads").unwrap();
    let raw_html: String = result.1;
    let html: Html = Html::parse_document(&raw_html);
    let selector = Selector::parse(".license-button").unwrap();

    let elements: Vec<&str> = html
        .select(&selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            if href.contains(".vbox-extpack") {
                Some(href)
            } else {
                None
            }
        })
        .collect();

    if let Some(link) = elements.first() {
        link.to_string()
    } else {
        String::from("No download link found")
    }
}

pub fn get_owasp_zap_link() -> String {
    let result = make_web_request("https://www.zaproxy.org/download/").unwrap();
    let raw_html: String = result.1;
    let html: Html = Html::parse_document(&raw_html);
    let selector = Selector::parse(r#"a[track-event="download.stable.win-64-i"]"#).unwrap();

    let elements: Vec<String> = html
        .select(&selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            if href.contains(".exe") {
                Some(href.to_string())
            } else {
                None
            }
        })
        .collect();

    if let Some(link) = elements.first() {
        link.to_string()
    } else {
        String::from("No download link found")
    }
    //
}

pub fn get_postgresql_link() -> String {
    let result =
        make_web_request_v2("https://www.enterprisedb.com/downloads/postgres-postgresql-downloads")
            .unwrap();
    let raw_html: String = result.1;
    let html: Html = Html::parse_document(&raw_html);

    // Selects only the most recent windows x64 build of postgresql verified by edb
    let selector =
        Selector::parse("tr.border-y.border-white:nth-child(1) td.text-center.py-4:nth-child(5) a")
            .unwrap();

    let elements: Vec<String> = html
        .select(&selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            // If the href is present ensure it contains the id based url
            if href.starts_with("https://sbp.enterprisedb.com/getfile.jsp?fileid=") {
                Some(href.to_string())
            } else {
                None
            }
        })
        .collect();

    // Return the first found link, or an error message
    elements
        .first()
        .cloned()
        .unwrap_or_else(|| String::from("No download link found"))
    //tr.border-y:nth-child(1) > td:nth-child(5) > a:nth-child(1)
}

pub fn get_postman_link() -> String {
    String::from("https://dl.pstmn.io/download/latest/win64")
}

pub fn get_python2_7_18_link() -> String {
    String::from("https://www.python.org/ftp/python/2.7.18/python-2.7.18.amd64.msi")
}

pub fn get_python3_8_10_link() -> String {
    String::from("https://www.python.org/ftp/python/3.8.10/python-3.8.10-amd64.exe")
}

pub fn get_python3_9_10_link() -> String {
    String::from("https://www.python.org/ftp/python/3.9.10/python-3.9.10-amd64.exe")
}

pub fn get_python3_10_10_link() -> String {
    String::from("https://www.python.org/ftp/python/3.10.10/python-3.10.10-amd64.exe")
}

pub fn get_python3_11_10_link() -> String {
    String::from("https://www.python.org/ftp/python/3.11.10/python-3.11.10-amd64.exe")
}

pub fn get_python3_12_9_link() -> String {
    String::from("https://www.python.org/ftp/python/3.12.9/python-3.12.9-amd64.exe")
}

pub fn get_python3_13_2_link() -> String {
    String::from("https://www.python.org/ftp/python/3.13.2/python-3.13.2-amd64.exe")
}

pub fn get_putty_link() -> String {
    let result =
        make_web_request_v2("https://www.chiark.greenend.org.uk/~sgtatham/putty/latest.html")
            .unwrap();
    let raw_html = result.1;
    let html: Html = Html::parse_document(&raw_html);
    let selector = Selector::parse("span.downloadfile > a:nth-child(1)").unwrap();

    let elements: Vec<&str> = html
        .select(&selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            if href.contains(".msi") && href.contains("/latest/w64/putty-64bit-") {
                Some(href)
            } else {
                None
            }
        })
        .collect();

    if let Some(link) = elements.first() {
        link.to_string()
    } else {
        String::from("No download link found")
    }
}

pub fn get_sublime_text_link() -> String {
    let result =
        make_web_request_v2("https://www.sublimetext.com/download_thanks?target=win-x64").unwrap();
    let raw_html = result.1;
    let html: Html = Html::parse_document(&raw_html);
    let selector = Selector::parse("a").unwrap();

    let elements: Vec<&str> = html
        .select(&selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            if href.contains("sublime_text_build_") && href.contains("x64_setup.exe") {
                Some(href)
            } else {
                None
            }
        })
        .collect();

    if let Some(link) = elements.first() {
        link.to_string()
    } else {
        String::from("No download link found")
    }
}

pub fn get_utm_link() -> String {
    String::from("https://getutm.app/downloads/utm-guest-tools-latest.iso")
}

pub fn get_visual_studio_link() -> String {
    String::from(
        "https://c2rsetup.officeapps.live.com/c2r/downloadVS.aspx?sku=community&channel=Release&version=VS2022&source=VSLandingPage&cid=2030",
    )
}

pub fn get_vscode_link() -> String {
    String::from("https://code.visualstudio.com/sha/download?build=stable&os=win32-x64")
}

pub fn get_vscodium_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://github.com/VSCodium/vscodium/releases/latest").unwrap();
    let raw_html = result.1;
    if result.0 != StatusCode::OK {
        return String::from("No download link found");
    } else {
        let html: Html = Html::parse_document(&raw_html);
        let selector: Selector = Selector::parse(r#"h1[data-view-component="true"]"#).unwrap();
        if let Some(version) = html
            .select(&selector)
            .filter_map(|href| href.text().next())
            .next()
        {
            let link = format!(
                "https://github.com/VSCodium/vscodium/releases/download/{version}/VSCodiumSetup-x64-{version}.exe"
            );
            link
        } else {
            String::from("No download link found")
        }
    }
}

pub fn get_webstorm_link() -> String {
    String::from("https://download.jetbrains.com/webstorm/WebStorm-2024.3.5.exe")
}

pub fn get_winget_link() -> String {
    String::from("https://aka.ms/getwingetpreview")
}

pub fn get_wireshark_link() -> String {
    let result = make_web_request_v2("https://www.wireshark.org/download.html").unwrap();
    let raw_html = result.1;
    let html: Html = Html::parse_document(&raw_html);
    let selector = Selector::parse("a").unwrap();

    let elements: Vec<&str> = html
        .select(&selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            if href.contains("dl.wireshark.org/win64/") && href.contains("-x64.exe") {
                Some(href)
            } else {
                None
            }
        })
        .collect();

    if let Some(link) = elements.first() {
        link.to_string()
    } else {
        String::from("No download link found")
    }

    //https://2.na.dl.wireshark.org/win64/Wireshark-4.4.5-x64.exe
}

pub fn get_xampp_link() -> String {
    let result = make_web_request_v2("https://www.apachefriends.org/download.html").unwrap();
    let raw_html = result.1;
    let html: Html = Html::parse_document(&raw_html);
    let selector = Selector::parse("a.button").unwrap();

    let elements: Vec<&str> = html
        .select(&selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            if href.contains("xampp-windows-x64-") && href.contains("VS16-installer.exe") {
                Some(href)
            } else {
                None
            }
        })
        .collect();

    if let Some(link) = elements.iter().nth(2) {
        link.to_string() // Installer for the most recent version of xampp
    } else {
        String::from("No download link found")
    }
}
// endregion: Programming Download Functions

// region: System Utilities Functions

pub fn get_seven_zip_link() -> String {
    let result: (StatusCode, String) = make_web_request("https://7-zip.org/download.html").unwrap();
    //let status: StatusCode = result.0; // Request status, used for debugging
    let raw_html: String = result.1; // Request response (HTML)
    //println!("{}", raw_html); // Used for debugging
    let html: Html = Html::parse_document(&raw_html);
    let selector: Selector = Selector::parse("a").unwrap();
    let link: Option<String> = html
        .select(&selector)
        .filter_map(|element| {
            element
                .value()
                .attr("href")
                .filter(|element| element.contains("a/7z"))
                .filter(|element| element.contains("x64.msi"))
                .map(|element| String::from(element))
        })
        .next();
    // println!("{:?}", link); // used for debugging
    let base_link = link.unwrap_or_else(|| String::from("No download link found"));
    if base_link == "No download link found" {
        return String::from("No download link found");
    } else {
        let link: String = format!("https://7-zip.org/{base_link}");
        link
    }
}

pub fn get_audacious_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://www.audacityteam.org/download/windows/").unwrap();
    //let status: StatusCode = result.0; // Request status, used for debugging
    let raw_html: String = result.1; // Request response (HTML)
    //println!("{}", raw_html); // Used for debugging
    let html: Html = Html::parse_document(&raw_html);
    let selector: Selector = Selector::parse("a").unwrap();
    let link: Option<String> = html
        .select(&selector)
        .filter_map(|element| {
            element
                .value()
                .attr("href")
                .filter(|element| element.contains("/releases/download/Audacity-"))
                .filter(|element| element.contains("audacity-win-"))
                .filter(|element| element.contains("-64bit.exe"))
                .map(|element| String::from(element))
        })
        .next();
    // println!("{:?}", link); // used for debugging
    link.unwrap_or_else(|| String::from("No download link found"))
}

// Function that downloads libreoffice needs to handle redirect to:
//https://mirrors.ukfast.co.uk/sites/documentfoundation.org/tdf/libreoffice/stable/25.2.1/win/x86/LibreOffice_25.2.1_Win_x86.msi
pub fn get_libreoffice_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://www.libreoffice.org/download/download-libreoffice/").unwrap();
    //let status: StatusCode = result.0; // Request status, used for debugging
    let raw_html: String = result.1; // Request response (HTML)
    //println!("{}", raw_html); // Used for debugging
    let html: Html = Html::parse_document(&raw_html);
    let selector: Selector = Selector::parse("a.dl_download_link").unwrap();
    let link: Option<String> = html
        .select(&selector)
        .filter_map(|element| {
            element
                .value()
                .attr("href")
                // .filter(|element| element.contains("/donate/dl/win-x86_64/"))
                // .filter(|element| element.contains("_Win_x86-64.msi"))
                .map(|element| String::from(element))
        })
        .next();
    // println!("{:?}", link); // used for debugging
    link.unwrap_or_else(|| String::from("No download link found"))
}

pub fn get_modern_csv_link() -> String {
    String::from("https://www.moderncsv.com/download-windows")
}

pub fn get_nomacs_link() -> String {
    String::from("https://github.com/nomacs/nomacs/releases/latest/download/nomacs-setup-x64.msi")
}

pub fn get_okular_link() -> String {
    //https://cdn.kde.org/ci-builds/graphics/okular/master/windows/

    let result: (StatusCode, String) =
        make_web_request("https://cdn.kde.org/ci-builds/graphics/okular/master/windows/").unwrap();
    //let status: StatusCode = result.0; // Request status, used for debugging
    let raw_html: String = result.1; // Request response (HTML)
    //println!("{}", raw_html); // Used for debugging
    let html: Html = Html::parse_document(&raw_html);
    let selector: Selector = Selector::parse("a").unwrap();
    let link: Option<String> = html
        .select(&selector)
        .filter_map(|element| {
            element
                .value()
                .attr("href")
                .filter(|element| element.ends_with(".exe"))
                .map(|element| String::from(element))
        })
        .next();
    // println!("{:?}", link); // used for debugging
    let filename = link.unwrap_or_else(|| String::from("No download link found"));
    if filename != "No download link found" {
        let link =
            format!("https://cdn.kde.org/ci-builds/graphics/okular/master/windows/{filename}");
        link
    } else {
        filename
    }
}

// pub fn get_photoshop_cs6_link() -> String {
//     String::from("https://example.com/photoshop_cs6")
// }

pub fn get_rufus_link() -> String {
    //https://github.com/pbatard/rufus/releases/latest
    let result: (StatusCode, String) =
        make_web_request("https://github.com/pbatard/rufus/releases/latest").unwrap();
    let raw_html = result.1;
    if result.0 != StatusCode::OK {
        return String::from("No download link found");
    } else {
        let html: Html = Html::parse_document(&raw_html);
        let selector: Selector = Selector::parse("span.ml-1").unwrap();
        if let Some(raw_version) = html
            .select(&selector)
            .filter_map(|href| href.text().next())
            .next()
        {
            let version = raw_version.trim();
            let partial_file_raw = raw_version.replace("v", "");
            let partial_filename = partial_file_raw.trim();
            let link: String = format!(
                "https://github.com/pbatard/rufus/releases/download/{version}/rufus-{partial_filename}_x86.exe"
            );
            link
        } else {
            String::from("No download link found")
        }
    }
}

pub fn get_vlc_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://www.videolan.org/vlc/download-windows.html").unwrap();
    //let status: StatusCode = result.0; // Request status, used for debugging
    let raw_html: String = result.1; // Request response (HTML)
    //println!("{}", raw_html); // Used for debugging
    let html: Html = Html::parse_document(&raw_html);
    let selector: Selector = Selector::parse(r#"a[id="downloadButton2"]"#).unwrap();
    let raw_link: Option<String> = html
        .select(&selector)
        .filter_map(|element| {
            element
                .value()
                .attr("href")
                .map(|element| String::from(element))
        })
        .next();
    // println!("{:?}", link); // used for debugging
    let raw_version = raw_link.unwrap_or_else(|| String::from("No download link found"));
    if raw_version == "No download link found" {
        return raw_version;
    }
    let version = raw_version
        .split("/")
        .nth(4)
        .unwrap_or_else(|| "No download link found");

    if !version.is_empty() {
        return format!("https://get.videolan.org/vlc/{version}/win32/vlc-{version}-win32.exe");
    } else {
        return String::from("No download link found");
    }
}

pub fn get_winrar_link() -> String {
    let result: (StatusCode, String) =
        make_web_request("https://www.rarlab.com/download.htm").unwrap();
    //let status: StatusCode = result.0; // Request status, used for debugging
    let raw_html: String = result.1; // Request response (HTML)
    //println!("{}", raw_html); // Used for debugging
    let html: Html = Html::parse_document(&raw_html);
    let selector: Selector = Selector::parse("a").unwrap();
    let raw_link: Option<String> = html
        .select(&selector)
        .filter_map(|element| {
            element
                .value()
                .attr("href")
                .filter(|href| href.contains("/rar/winrar-x64-"))
                .map(|element| String::from(element))
        })
        .next();
    // println!("{:?}", link); // used for debugging
    let filename = raw_link.unwrap_or_else(|| String::from("No download link found"));
    if filename == "No download link found" {
        return String::from("No download link found");
    } else {
        let link = format!("https://www.rarlab.com{filename}");
        return link;
    }
}

// endregion: System Utilities Functions
