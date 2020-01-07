pub mod musicbox;

use std::env;
use musicbox::*;

fn main() {
    let args: Vec<_> = env::args().collect();


    if (args.len() < 2){
        print!("Usage {} display/find\n\n", args[0]);
    }else{
        if (args[1] == "display"){
            if (args.len() == 3){
                let mut collection: Collection = Collection::new();
                collection.parseFile(std::env::args().nth(2).unwrap());
                collection.display_albums();
            }else{
                print!("Usage {} display <collectionname>\n", args[0]);
            }
        }else if (args[1] == "find"){
            if (args.len() == 4){
                let mut collection: Collection = Collection::new();
                collection.parseFile(std::env::args().nth(2).unwrap());
                collection.display_album(
                  (&std::env::args().nth(3).unwrap()[..])
                );
            }else{
                print!("Usage {} find <collectionname> <searchterm>\n",args[0]);
            }
        }else if (args[1] == "add"){
            let collection: Collection = Collection::new();
            collection.parseFile(std::env::args().nth(2).unwrap());
            collection.add_to_album(std::env::args().nth(
        }else{
            for i in std::env::args(){
                print!("{}\n", i);
            }
            print!("Usage {} add/display\n", args[0]);
        }
   }
}
