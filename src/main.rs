mod parser;
mod traits;
mod request;

use std::{fs};
use clap::Parser;
use crate::parser::bencode::parse_bencode;
use crate::parser::torrent_file::TorrentFile;
use crate::parser::peers::AnnounceResponse;
use crate::traits::from_bencode::CreateFromBencode;
use crate::request::handshake::Handshake;


#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();
    let bencode_byte = fs::read(&args.file)?;

    let bencode_input = parse_bencode(&bencode_byte);
    let torrent = TorrentFile::new_from_bencode(&bencode_input.0)
        .expect("Failed to parse TorrentFile");
    println!("requesting peers...");
    let response = reqwest::get(torrent.build_tracker_url()?)
        .await?;
    let body_bytes = response.bytes().await?;
    let announce_response = parse_bencode(&body_bytes);
    let announce = AnnounceResponse::parse(&announce_response.0);
    println!("{:?}", announce);

    for peer in &announce.peers {
        let peer_id = *b"01234567890123456789";
        let handshake = Handshake::new(torrent.info_hash, peer_id);
        let peer_hand_result = handshake.shake(peer).await;
        match peer_hand_result {
            Ok(peer_hand) => {
                println!("handshake done {:?}", peer_hand);
                let peer_handshake = Handshake::parse(peer_hand);
                println!("handshake done {:?}", peer_handshake);
            }
            Err(e) => continue
        }
    }

    Ok(())
}