pub mod musicbox;

use std::env;
use musicbox::*;
use clap::{Arg, App, SubCommand};

fn main(){
    let matches = App::new("Musicbox")
      .version("0.1").author("SuedeGently")
      .subcommand(SubCommand::with_name("display")
        .arg(Arg::with_name("collection")
          .required(true)
          .index(1)
          .help("Collection to be displayed")))
      .get_matches();
    if let display = matches.subcommand_matches("display").unwrap(){
        let mut collection: Collection = Collection::new();
        collection.parseFile(
          display.value_of("collection").unwrap().to_string()
        );
        collection.display_albums();
    }
}
