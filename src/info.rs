const NAME: &str = "gambero";
const DESCRIPTION: &str =
    "🦐 Command-line interface (CLI) for Kraken exchange WebSockets API | Written in Rust";
const VERSION: &str = "0.1.2";
const AUTHOR: &str = "Matteo Pisani <matteo.pisani.91@gmail.com>";
const REPOSITORY: &str = "https://github.com/xonoxitron/gambero";

pub fn get_crate_info() -> String {
    format!(
        "Name: {}\r\nDescription: {}\r\nVersion: {}\r\nAuthor: {}\r\nRepository: {}",
        NAME, DESCRIPTION, VERSION, AUTHOR, REPOSITORY
    )
    .to_string()
}

pub fn print_crate_info() {
    println!("-- CRATE INFO --\r\n{}\r\n", get_crate_info());
}

pub fn print_crate_usage() {
    println!("-- CRATE USAGE --\r\nKraken public API interaction:\tgambero public\r\nKraken private API interaction:\tgambero private <kraken_api_key> <kraken_api_secret>\r\n");
}