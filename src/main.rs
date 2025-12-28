use std::env;
use utils::*;

mod utils;

fn print_usage() {
    println!("{:<15} | {}", "Command", "Description");
    println!("{:-<15}-|-{:-<35}", "", ""); // separator

    println!("{:<15} | {}", "ls", "Lists the items in the todo list");
    println!("{:<15} | {}", "cl", "Clears/empties the list");
    println!("{:<15} | {}", "done <id>", "Toggles done state");
    println!("{:<15} | {}", "rm <id>", "Removes the todo item");
    println!("{:<15} | {}", "add <todo>", "Adds a todo item");
    println!("{:<15} | {}", "mod <id> <todo>", "Modifies the todo item");
    println!("{:<15} | {}", "del <path>", "Deletes a todo file");
}


fn main() {
    let args: Vec<String> = env::args().collect(); // we collect the arguments here for our todo_list

    if args.len() < 2{
          println!("Use `help` for usage of the rustodo-cli.");
          return;
    }

    // ensures file is not created when we call help
    if args[1].as_str() =="help" {
        print_usage();
        return;
    }

    let mut is_del : bool = false; // only save if the file was not deleted

    let file_path = "list.todo".to_string(); // Simplified path for testing

    let mut items = load(file_path.clone()).unwrap_or_else(|_| Vec::new()); // we start with empty list if the file does not exist

    match args[1].as_str() {
           "add" => {
               if args.len() < 3 { println!("Error: Missing task"); return; }
               let todo = args[2..].join(" ");
               add(&mut items, todo);
               println!("ADDED!");
           }
           "ls" => {
               list(&items);
           }
           "mod" => {
               if args.len() < 4 { println!("Usage: mod <id> <new_todo>"); return; }
                let id: u64 = args[2].parse().unwrap();
                let todo = args[3..].join(" ");
                modify(&mut items, id, todo);
           }
           "done" => {
               if args.len() < 3 { println!("Error: Missing ID"); return; }
               let id: u64 = args[2].parse().unwrap();
               toggle_done(&mut items, id);
           }
           "rm" => {
               if args.len() < 3 { println!("Error: Missing ID"); return; }
               let id: u64 = args[2].parse().unwrap();
               remove(&mut items, id);
           }
           "cl" => {
               items = Vec::new();
               // clear the todo list
           }
           "del" => {
               if args.len() < 3 { println!("Error: Missing Path"); return; }
               let path: String = args[2].to_string();
               delete(path);
               is_del = true;
           }
           _ => {
               println!("Unknown command -> use `help` for usage.");
           }
       }

       if !is_del {
           save(&items, file_path).expect("Failed to save file"); // use move now
       }


}
