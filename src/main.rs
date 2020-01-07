pub mod musicbox;

use std::env;
use musicbox::*;
use clap::{Arg, App, SubCommand};

fn main(){
    let matches = App::new("Musicbox")
      .version("0.1").author("SuedeGently")
      .subcommand(SubCommand::with_name("display")
        .arg(Arg::with_name("target")
          .help("Collection to be displayed")
          .index(1)))
      .subcommand(SubCommand::with_name("find")
        .arg(Arg::with_name("target")
          .required(true)
          .index(1)
          .help("Album to be found"))
        .arg(Arg::with_name("collection")
          .help("Collection to be targeted")
          .index(2)))
      .get_matches();

    if let Some(display) = matches.subcommand_matches("display"){
        let mut collection: Collection = Collection::new();
        collection.parseFile(
          display.value_of("collection")
          .unwrap_or("content/Albums.txt")
          .to_string()
        );
        collection.display_albums();
    }else if let Some(find) = matches.subcommand_matches("find"){
        println!(
          "Finding {} in {}",
          find.value_of("target").unwrap_or("it boke"),
          find.value_of("collection").unwrap_or("content/Albums.txt")
            .to_string()
        );
        let mut collection: Collection = Collection::new();
        collection.parseFile(
          find.value_of("collection")
          .unwrap_or("content/Albums.txt")
          .to_string()
        );
        collection.display_album(find.value_of("target").unwrap());
    }
}
