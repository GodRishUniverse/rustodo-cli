use std::env;
use utils::*;

mod utils;


fn main() {
    let args: Vec<String> = env::args().collect(); // we collect the arguments here for our todo_list
    let mut items = Vec::new();

    if args.len() < 2 {
        println!("Usage:");
        println!("  add <todo>");
        println!("  mod <id> <todo>");
        println!("  list");
        println!("  done <id>");
        println!("  remove <id>");
        return;
    }

    match args[1].as_str() {
           "add" => {
               let todo = args[2..].join(" ");
               add(&mut items, todo);
               println!("ADDED!");
           }
           "list" => {
               list(&items);
           }
           "mod" => {
                let id: u64 = args[2].parse().unwrap();
                let todo = args[3..].join(" ");
                modify(&mut items, id, todo);
           }
           "done" => {
               let id: u64 = args[2].parse().unwrap();
               toggle_done(&mut items, id);
           }
           "remove" => {
               let id: u64 = args[2].parse().unwrap();
               remove(&mut items, id);
           }
           _ => {
               println!("Unknown command");
           }
       }


}
