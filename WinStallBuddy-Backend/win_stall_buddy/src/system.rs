use crate::links;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Utility {
    SevenZip,
    Audacious,
    LibreOffice,
    ModernCSV,
    NoMacs,
    Okular,
    //PhotoshopCS6,
    Rufus,
    VLC,
    WinRAR,
}

impl Utility {
    pub fn get_link(&self) -> Option<String> {
        match self {
            Utility::SevenZip => Some(links::get_seven_zip_link()),
            Utility::Audacious => Some(links::get_audacious_link()),
            Utility::LibreOffice => Some(links::get_libreoffice_link()),
            Utility::ModernCSV => Some(links::get_modern_csv_link()),
            Utility::NoMacs => Some(links::get_nomacs_link()),
            Utility::Okular => Some(links::get_okular_link()),
            //Utility::PhotoshopCS6 => Some(links::get_photoshop_cs6_link()),
            Utility::Rufus => Some(links::get_rufus_link()),
            Utility::VLC => Some(links::get_vlc_link()),
            Utility::WinRAR => Some(links::get_winrar_link()),
        }
    }
}
