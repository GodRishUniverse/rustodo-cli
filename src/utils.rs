use std::fs::*;
use std::io::{BufRead, BufReader, BufWriter, Result, Write};
use std::path::Path;

/*
 * We need the #[derive(Debug)] to automatically implements the Debug trait - allows to format a value for debugging purposes using the {:?} or {:#?} format in println
 */

// public struct for item in todo list
#[derive(Debug)]
pub struct Item {
    pub id: u64,
    pub todo: String,
    pub done: bool,
}

pub fn modify(items: &mut Vec<Item>, id: u64, todo: String) {
    match items.binary_search_by_key(&id, |item| item.id) {
        Ok(index) => items[index].todo = todo,
        Err(_) => println!("Not found"),
    }
}

pub fn add(items: &mut Vec<Item>, todo: String) {
    // add
    // let mut id = 0;
    // if (items.len() as u64 == 0){
    //     id =1;
    // } else{
    //     id = items[items.len()-1].id+1;
    // }

    //Same as above but more rusty
    let id = match items.last() {
        Some(item) => item.id + 1,
        None => 1,
    };

    items.push(Item {
        id,
        todo,
        done: false,
    });
}

pub fn list(items: &Vec<Item>) {
    for item in items {
        let status = if item.done { "[x]" } else { "[ ]" };
        println!("{} {} {}", item.id, status, item.todo);
    }
}

pub fn toggle_done(items: &mut Vec<Item>, id: u64) {
    // change the toggle -> need to specify id
    match items.binary_search_by_key(&id, |item| item.id) {
        Ok(index) => items[index].done = !items[index].done,
        Err(_) => println!("Not found"),
    }
}

pub fn remove(items: &mut Vec<Item>, id: u64) {
    // remove from our list -> specify id
    match items.binary_search_by_key(&id, |item| item.id) {
        Ok(index) => {
            items.remove(index);
        }
        Err(_) => {
            println!("Not found");
        }
    }
}

pub fn save(items: &Vec<Item>, filepath: String) -> Result<()> {
    // save the todo list that we have in Vector of Items

    let file = File::create(&filepath)?;
    let mut writer = BufWriter::new(file);

    let mut c = 1;
    for item in items {
        writeln!(writer, "{},{},{}", c, item.done, item.todo)?;
        c += 1; // so that ID is always sorted
    }

    writer.flush()?;

    Ok(())
}

pub fn load(filepath: String) -> Result<Vec<Item>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut items = Vec::new();

    for line in reader.lines() {
        let line = line?; // Handle potential I/O errors for each line
        let values: Vec<&str> = line.split(',').collect(); // separate the line

        if values.len() == 3 {
            items.push(Item {
                // 3. Name the fields and handle parsing
                id: values[0].trim().parse().unwrap_or(0),
                done: values[1].trim().parse().unwrap_or(false),
                // 4. Convert &str to String
                todo: values[2].trim().to_string(),
            });
        }
    }

    Ok(items)
}

pub fn delete(filepath: String) {
    if Path::new(&filepath)
        .extension()
        .map_or(false, |ext| ext == "todo")
    {
        match remove_file(&filepath) {
            Ok(()) => {
                println!("File removed successfully");
            }
            Err(_) => {
                eprintln!("Error removing file.");
            }
        }
    } else {
        println!("Can only delete `.todo` files created by rustodo-cli");
    }
}
