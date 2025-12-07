mod parser;

use std::{fs};
use crate::parser::bencode::parse_bencode;
use crate::parser::torrent_file::TorrentFile;


fn main() -> std::io::Result<()> {
    let bencode_byte = fs::read("/Users/teospadotto/Documents/project/Rust/study/resource/debian-12.10.0-amd64-netinst.iso.torrent")?;
    let bencode_input = parse_bencode(&bencode_byte);
    let torrent= TorrentFile::new_from_bencode(&bencode_input.0);
    println!("{:?}", torrent.unwrap());

    Ok(())
}