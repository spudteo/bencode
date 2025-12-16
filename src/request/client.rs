use std::io::Error;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use crate::parser::peers::Peer;
use crate::parser::torrent_file::TorrentFile;
use crate::request::handshake::Handshake;


use thiserror::Error;
use tokio::time::error::Elapsed;
use crate::request::torrent_message::TorrentMessage;

#[derive(Debug, Error)]
pub enum ClientError {

    #[error("problem with handshake")]
    Handshake,

    #[error("connection timeout")]
    Timeout,
    #[error("input non valido: {0}")]
    InvalidInput(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<Elapsed> for ClientError {
    fn from(_: Elapsed) -> Self {
        ClientError::Timeout
    }
}


pub struct Client {
    torrent_file: TorrentFile,
    peer: Peer,
    client_peer_id: [u8; 20]
}


impl Client {
    pub fn new(torrent_file: TorrentFile, peer: Peer) -> Client {
        let client_per_id = *b"01234567890123456789";
        Self{torrent_file, peer, client_peer_id: client_per_id }
    }

    pub async fn handshake_done(stream : &mut TcpStream, handshake: Handshake) -> Result<bool, Error> {
        let data = handshake.to_bytes();
        stream.write_all(&data).await?;
        let mut buf = [0u8; 68];
        let n = stream.read(&mut buf).await?;
        if n > 0 {
            Ok(true)
            //check that handshake is ok in buf
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "peer closed connection without sending handshake, buff looks empty",
            ))
        }
    }


    pub async fn download_from_peer(self,piece_id: u32) -> Result<Vec<u8>, ClientError> {
        //create tcp connection
        let socket = SocketAddr::new(self.peer.ip_addr, self.peer.port_number as u16);
        let mut stream = timeout(
            Duration::from_secs(5),
            TcpStream::connect(socket)
        ).await??;
        //handshake
        let handshake = Handshake::new(self.torrent_file.info_hash, self.client_peer_id);
        if !Client::handshake_done(&mut stream, handshake).await? {
            return Err(ClientError::Handshake);
        }
        let mut init_buf = [0u8; 4];
        let n = stream.read(&mut init_buf).await?;
        let message_length = match n == 4 {
            true => u32::from_be_bytes(init_buf[0..4].try_into().unwrap()) as usize,
            false => 0,
        };
        let mut message_buf = vec![0u8; message_length];
        stream.read_exact(&mut message_buf).await?;
        let message = TorrentMessage::read(&message_buf);
        println!("{:?}", message);
        match message {
            TorrentMessage::Bitfield {bitfield} => Ok(bitfield),
            _ => Err(ClientError::InvalidInput("invalid torrent message it is not a bitfield message".to_string())),
        }
    }


}