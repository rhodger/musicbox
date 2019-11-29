pub mod musicbox;

use std::env;
use musicbox::*;

fn main() {
    let args: Vec<_> = env::args().collect();


    if (args.len() < 2){
        print!("Usage {} add arguments\n", args[0]);
    }else{
        if (args[1] == "display" && args.len() == 3){
            let mut collection: Collection = Collection::new();
            collection.parseFile(std::env::args().nth(2).unwrap());
            collection.display_albums();
        }else if (args[1] == "find" && args.len() == 4){
            let mut collection: Collection = Collection::new();
            collection.parseFile(std::env::args().nth(2).unwrap());
            collection.display_album((&std::env::args().nth(3).unwrap()[..]));
        }else if(args[1] == "display" && args.len() == 2){
            let mut collection: Collection = Collection::new();
            collection.parseFile(std::env::args().nth(2).unwrap());
            collection.display_albums();
        }else{
            for i in std::env::args(){
                print!("{}\n", i);
            }
            print!("Usage {} add arguments\n", args[0]);
        }
   }
}
