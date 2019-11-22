pub mod musicbox;

use std::env;
use musicbox::*;

fn main() {
    let args: Vec<_> = env::args().collect();


    if (args.len() < 2){
        print!("Usage {} add arguments\n", args[0]);
    }else{
        if (args[1] == "view_albums" && args.len() == 3){
            let mut collection: Collection = Collection::new();
            collection.parseFile(std::env::args().nth(2).unwrap());
            collection.display_albums();
        }else{
            print!("Usage {} add arguments\n", args[0]);
        }
   }
}
