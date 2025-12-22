mod parser;
mod request;
mod traits;

use crate::request::client::Client;
use crate::traits::from_bencode::CreateFromBencode;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();
    let bencode_byte = fs::read(&args.file)?;
    let one_client = Client::new(&bencode_byte);
    one_client.download_torrent().await?;
    Ok(())
}
