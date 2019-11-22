//! A selection of tools for managing song data.

use std::io::{BufReader,BufRead};
use std::fs::File;
use regex::Regex;
use str;



/// Holds a single Song.
struct Song{
    name: String,
    artist: String,
    length: u16
}

/// Holds a single Album.
struct Album{
    name: String,
    artist: String,
    length: u8,
    songs: Vec<Song>
}

/// Holds several Albums and allows for public interaction with them.
pub struct Collection{
    length: u16,
    albums: Vec<Album>
}



impl Song{
    /// Constructs a new Song.
    fn new(n: String, a: String, l: u16) -> Song{
        let x = Song{
            name: n,
            artist: a,
            length: l
        };
        return x;
    }
}

impl Album{
    /// Adds a new Song to this Album.
    fn add(&mut self, x: Song){
        self.songs.push(x);
        self.length += 1;
    }

    
    /// Constructs a new Album.
    fn new(n: String, a: String) -> Album{
        let new = Album{
            name: n,
            artist: a,
            length: 0,
            songs: Vec::new()
        };
        return new;
    }
}

impl Collection{
    /// Returns the amount of songs in this Collection.
    pub fn getLength(&self) -> u16{
        return self.length;
    }

    /// Adds a new Album to this Collection.
    fn add(&mut self, x: Album){
        self.albums.push(x);
        self.length += 1;
    }

    /// Parses the file at the given path and adds its contents to this
    /// Collection, provided it is formatted correctly.
    /// 
    /// The file must be formatted as a list of songs separated by Album
    /// titles/artists. For more information about the correct way of
    /// formatting these files, refer to the example content file 'Albums.txt'.
    /// 
    /// # Examples
    /// 
    /// For the below appropriately formatted file 'Albums.txt'
    /// ```text
    /// Jimmi Hendrix Experience : Are you Experienced?
    /// 0:03:22 - Foxy Lady
    /// 0:03:46 - Manic Depression
    /// 0:03:53 - Red House
    /// 0:02:35 - Can You See Me
    /// 0:03:17 - Love or Confusion
    /// 0:03:58 - I Don't Live Today
    /// 0:03:14 - May This Be Love
    /// 0:02:47 - Fire
    /// 0:06:50 - Third Stone from the Sun
    /// ```
    /// you could run
    /// ```
    /// let collection: Collection = Collection::new();
    /// collection.parseFile("Albums.txt");
    /// ```
    pub fn parseFile(&mut self, path: String){
        let file = BufReader::new(File::open(&path).unwrap());

        let mut album: Album = Album::new(
          String::from("DEFAULT"),
          String::from("DEFALT")
        );
        for line in file.lines(){
            let nextLine: String = line.unwrap();
            if (Regex::new(r"-").unwrap().is_match(&nextLine)){
                let data: Vec<&str> =
                  Regex::new(r"\s{1}-\s{1}").unwrap()
                  .split(&nextLine).collect();
                let song: Song = Song::new(
                  data[1].to_string(),
                  album.artist.to_string(),
                  convert_time(data[0])
                );
            }else{
                print!("Album complete; length: {}\n", album.length);
                for song in &album.songs{
                    print!("{}\n", song.name);
                }
                self.add(album);
                album = Album::new(
                  String::from("DEFAULT"),
                  String::from("DEFALT")
                );
            }
        }
    }

    /// Displays a save file as a list of albums.
    pub fn display_albums(&self){
        for album in &self.albums{
            print!("{}\n", album.name);
        }
    }

    pub fn display_album(&self, album: &str){
        //Empty for now
    }

    /// Constructs a new Collection
    pub fn new() -> Collection{
        let new:Collection = Collection{
            length: 0,
            albums: Vec::new()
        };
        return new;
    }
}


/// Converts a string representation of a time into its numerical equivalent.
/// 
/// Taking a string of the format "HH:MM:SS", this function returns that same
/// time in seconds as a u16.
/// 
/// # Examples
/// 
/// ```
/// let time: &str = String::from("00:1:20");
/// print("{}", convert_time(time));
/// ```
fn convert_time(time: &str) -> u16{
    let values: Vec<&str> = Regex::new(r":").unwrap().split(time).collect();
    let totalTime: u16 = (
      (values[0].parse::<u16>().unwrap() * 60 * 60) +
      (values[1].parse::<u16>().unwrap() * 60) +
      values[2].parse::<u16>().unwrap()
    );
    return totalTime;
}