use std::env;
use utils::*;

mod utils;


fn main() {
    let args: Vec<String> = env::args().collect(); // we collect the arguments here for our todo_list

    if args.len() < 2 {
        println!("Usage:");
        println!("  add <todo>");
        println!("  list");
        println!("  done <id>");
        println!("  remove <id>");
        return;
    }


}
