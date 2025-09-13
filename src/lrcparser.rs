use regex::Regex;

pub struct LRCParser {
     text: String,

     // info
     song_title: String,
     artist: String,
     album: String,
     author: String,
     lyricist: String,
     length: i32,
     lrc_author: String,
     tool: String,
     lyrics_lines_count: u64,

     // regexes
     lyric_regex: String,
     tag_regex: String,
     length_regex: String,
}

impl LRCParser {
     pub fn new(_text: String) -> LRCParser {
          LRCParser { 
               text: _text, 

               // info
               song_title: String::new(),
               artist: String::new(),
               album: String::new(),
               author: String::new(),
               lyricist: String::new(),
               length: 0,
               lrc_author: String::new(),
               tool: String::new(),
               lyrics_lines_count: 0,

               // regexes
               lyric_regex: String::from(r"\[([0-9][0-9]):([0-9][0-9])\.([0-9][0-9])\](.*)$"),
               tag_regex: String::from(r"\[(.+?): *(.+)\]$"),
               length_regex: String::from(r"(\d{1, 2}):(\d{1, 2})$")
          }
     }

     pub fn get_song_title(&self) -> &String {
          return &self.song_title;
     }

     pub fn get_song_artist(&self) -> &String {
          return &self.artist;
     }

     pub fn get_song_album(&self) -> &String {
          return &self.album;
     }

     pub fn get_song_author(&self) -> &String {
          return &self.author;
     }

     pub fn get_song_lyricist(&self) -> &String {
          return &self.lyricist;
     }

     pub fn get_song_length_seconds(&self) -> i32 {
          return &self.length % 60;
     }

     pub fn get_song_length_minutes(&self) -> i32 {
          return &self.length / 60;
     }

     pub fn get_lrc_author(&self) -> &String {
          return &self.lrc_author;
     }

     pub fn get_lrc_tool(&self) -> &String {
          return &self.tool;
     }

     pub fn get_lyrics_lines_count(&self) -> u64 {
          return self.lyrics_lines_count;
     }

     fn do_tag(&mut self, tag_name: &str, value: &str){
          match tag_name {
               "ti" => {
                    self.song_title = String::from(value);
               },
               "ar" => {
                    self.artist = String::from(value);
               },
               "al" => {
                    self.album = String::from(value);
               },
               "au" => {
                    self.author = String::from(value);
               },
               "lr" => {
                    self.lyricist = String::from(value);
               },
               "length" => {
                    let length_regex = Regex::new(&self.length_regex).unwrap();
                    if let Some(caps) = length_regex.captures(value){
                         // it's guaranteed to be a valid integer - regex validates it
                         let minutes = caps[1].parse::<i32>().unwrap();
                         let seconds = caps[2].parse::<i32>().unwrap();
                         self.length = minutes * 60 + seconds;
                    } else {
                         println!("parse warning: got tag `length` with invalid value");
                    }
               },
               "by" => {
                    self.lrc_author = String::from(value);
               },
               "re" => {
                    self.tool = String::from(value);
               },
               "tool" => {
                    self.tool = String::from(value);
               },
               _ => {
                    println!("parse warning: got unknown tag `{}`", tag_name);
               }
          }
     }

     pub fn parse<'lines>(&mut self) {
          let lines = self.text.split("\n").collect::<Vec<&str>>();
          let lyric_regex = Regex::new(&self.lyric_regex).unwrap();
          let tag_regex = Regex::new(&self.tag_regex).unwrap();
          let mut tags: Vec<(String, String)> = Vec::new();

          for line in lines.iter(){
               if line.starts_with('#') || line.starts_with(';') { // comment
                    continue;
               }

               if let Some(_caps) = lyric_regex.captures(line){
                    self.lyrics_lines_count += 1;
               }
               else if let Some(caps) = tag_regex.captures(line){
                    tags.push((
                         String::from(&caps[1]), 
                         String::from(&caps[2])
                    ));
               }
          }

          for tag in tags.iter(){
               self.do_tag(&tag.0, &tag.1);
          }
     }
}