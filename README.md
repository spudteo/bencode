I started this project because I wanted to learn Rust. 

After watching a lot of youtube videos, one common advice is to built something on your own from scratch if you want to learn the language. 
Since I don't use Rust within my job, I decided to follow this advice. 

During the same period I learned about Plex, that let you build your own media system, and the majority of the files are coming from torrent. 
So I decided to look out for a torrent project and I find a blog about someone building his own torrent client in Go. 
I then decided to build my own in Rust. 
For sure better implementation are already present but I just wanted to learn something and have fun, and since torrent involved communication exchange it attracts me because is always awesome to receive a message from another computer on the network.


The main resources that I used was this Go blog: https://blog.jse.li/posts/torrent/
and the "official" specification for torrent, I wasn't able to find an RFC, https://wiki.theory.org/BitTorrentSpecification#Tracker_HTTP.2FHTTPS_Protocol.
For Rust I am following the official book: https://doc.rust-lang.org/book/.

I am not just translating from Go in Rust, I am using the blog mainly because it is easy to follow, and it gives hint for the minimal stuff that the protocol needs in order to work. 

In order to learn the language it doesn't make sense to use GenAi tool, so I am only making question regarding the syntax and making the implementation on my own even if a better approach could be done.   