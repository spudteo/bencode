mod parser;
mod traits;
mod request;

use std::{fs};
use crate::parser::bencode::parse_bencode;
use crate::parser::torrent_file::TorrentFile;
use crate::parser::peers::AnnounceResponse;
use crate::traits::from_bencode::CreateFromBencode;
use crate::request::handshake::Handshake;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Leggi file torrent
    let bencode_byte = fs::read("/Users/teospadotto/Documents/project/Rust/study/resource/debian-12.10.0-amd64-netinst.iso.torrent")
        .expect("Failed to read torrent file");

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
    let peer_id = *b"01234567890123456789";
    let handshake = Handshake::new(torrent.info_hash, peer_id);


    let peer_hand = handshake.shake(&announce.peers[0]).await.expect("TODO: panic message");

    println!("handshake done {:?}", peer_hand);

    let peer_handshake = Handshake::parse(peer_hand);

    println!("handshake done {:?}", peer_handshake);

    Ok(())
}