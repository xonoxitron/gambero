use async_std::prelude::*;
use async_std::{io, task};
use async_tungstenite::tungstenite::protocol::Message;
use futures::channel::mpsc::UnboundedSender;
use std::env;

use squalo;

pub struct Config {
    pub stream_type: String,
    pub kraken_api_key: String,
    pub kraken_api_secret: String,
}

fn help() {
    crate::info::print_crate_usage();
    std::process::exit(0);
}

fn get_args() -> Config {
    let args_count = env::args().len();
    if args_count == 1 || args_count > 4 {
        help();
    }
    let stream_type: String = env::args().nth(1).unwrap_or_default();
    let mut kraken_api_key: String = String::new();
    let mut kraken_api_secret: String = String::new();
    if stream_type != "public" {
        if stream_type == "private" {
            kraken_api_key = env::args().nth(2).unwrap_or_default();
            kraken_api_secret = env::args().nth(3).unwrap_or_default();
            if kraken_api_key == "" || kraken_api_secret == "" {
                println!(
                    "\r\n-- ERROR --\r\n{}\r\n",
                    "Both \"kraken_api_key\" and \"kraken_api_secret\" must be issued in \"private\" mode."
                );
                help();
            }
        } else {
            help();
        }
    }
    Config {
        stream_type: stream_type,
        kraken_api_key: kraken_api_key,
        kraken_api_secret: kraken_api_secret,
    }
}

async fn read_stdin(tx: UnboundedSender<Message>) {
    let mut stdin = io::stdin();
    loop {
        let mut buf = vec![0; 2048];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}

fn callback(result: String) {
    println!("{}", result);
}

pub async fn run() {
    let config = get_args();
    let (tx, rx) = squalo::create_communication_channel();
    if config.stream_type == "private" {
        squalo::set_kraken_api_credentials(config.kraken_api_key, config.kraken_api_secret);
        let token = squalo::get_websockets_token().await;
        println!(r#"{{"token":"{}"}}"#, token);
    }
    squalo::attach_websockets_stream(callback, config.stream_type, rx);
    task::spawn(read_stdin(tx)).await;
}
