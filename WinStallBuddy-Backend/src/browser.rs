use crate::links;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Browser {
    Brave,
    Chrome,
    Chromium,
    Edge,
    Firefox,
    Librewolf,
    Opera,
    OperaGX,
    PaleMoon,
    SeaMonkey,
    Tor,
    Vivaldi,
    Waterfox,
}

impl Browser {
    pub fn get_link(&self) -> Option<String> {
        match self {
            Browser::Brave => Some(links::get_brave_link()),
            Browser::Chrome => Some(links::get_chrome_link()),
            Browser::Chromium => Some(links::get_chromium_link()),
            Browser::Edge => Some(links::get_edge_link()),
            Browser::Firefox => Some(links::get_firefox_link()),
            Browser::Librewolf => Some(links::get_librewolf_link()),
            Browser::Opera => Some(links::get_opera_link()),
            Browser::OperaGX => Some(links::get_operagx_link()),
            Browser::PaleMoon => Some(links::get_palemoon_link()),
            Browser::SeaMonkey => Some(links::get_seamonkey_link()),
            Browser::Tor => Some(links::get_tor_link()),
            Browser::Vivaldi => Some(links::get_vivaldi_link()),
            Browser::Waterfox => Some(links::get_waterfox_link()),
        }
    }
}
