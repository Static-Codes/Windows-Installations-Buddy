mod browser;
mod cmd;
mod gaming;
mod links;
mod programming;
mod system;
mod utils;

fn test() {
    let app: gaming::Gaming = gaming::Gaming::RobloxLauncher;
    match app.get_link() {
        // link is returned from download_link() as either a String or None and is handled accordingly.
        Some(link) => println!("Download link found for {:?}: {}", app, link),
        None => println!("No download link found for {:?}.", app),
    }

    let browser: browser::Browser = browser::Browser::SeaMonkey;
    match browser.get_link() {
        // link is returned from download_link() as either a String or None and is handled accordingly.
        Some(link) => println!("Download link found for {:?}: {}", browser, link),
        None => println!("No download link found for {:?}.", browser),
    }

    let programming: programming::Programming = programming::Programming::XAMPP;
    match programming.get_link() {
        Some(link) => println!("Download link found for {:?}: {}", programming, link),
        None => println!("No download link found for {:?}", programming),
    }

    let system: system::Utility = system::Utility::WinRAR;
    match system.get_link() {
        Some(link) => println!("Download link found for {:?}: {}", system, link),
        None => println!("No download link found for {:?}", system),
    }

    let arg: String = utils::get_arg();
    let arg_enum: utils::ValidStarterArg = utils::map_arg_str_to_enum(arg);
    utils::handle_starter_arg(arg_enum);
}

fn main() {
    test();
}
