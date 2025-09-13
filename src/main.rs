pub mod lrcparser;

use std::{io::Read, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect(); 
    if args.len() < 2 {
        println!("lrcmd: no filename provided");
        return;
    }

    let file = fs::File::open(&args[1]);
    match file {
        Ok(mut file) => {
            let mut text = String::new();
            if let Err(error) = file.read_to_string(&mut text) {
                println!("Read error: {}", error);
                return;
            }

            let mut parser = lrcparser::LRCParser::new(text);
            parser.parse();
            println!("{}", args[1]);
            println!("title: {}", parser.get_song_title());
            println!("artist: {}", parser.get_song_artist());
            println!("album: {}", parser.get_song_album());
            println!("author: {}", parser.get_song_author());
            println!("lyricist: {}", parser.get_song_lyricist());
            println!("length: {}:{}", parser.get_song_length_minutes(), parser.get_song_length_seconds());
            println!("lrc author: {}", parser.get_lrc_author());
            println!("tool: {}", parser.get_lrc_tool());
            println!("lyrics lines: {}", parser.get_lyrics_lines_count());
        },
        Err(error) => {
            println!("Can't open file: {}", error);
        }
    }
}
