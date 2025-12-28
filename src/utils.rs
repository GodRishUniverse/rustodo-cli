// public struct for item in todo list

/*
 * We need the #[derive(Debug)] to automatically implements the Debug trait - allows to format a value for debugging purposes using the {:?} or {:#?} format in println
 */
#[derive(Debug)]
pub struct Item {
    pub id: u64,
    pub todo: String,
    pub done : bool,
}


pub fn modify(id: u64) {

}

pub fn add(items: &mut Vec<Item>, todo: String){
    // add
    let mut id = 0;
    if (items.len() as u64 == 0){
        id =1;
    } else{
        id = items[items.len()-1].id+1;
    }


    items.push(Item{id, todo, done: false});
}

pub fn list(items: &Vec<Item>) {
    for item in items {
        let status = if item.done { "[x]" } else { "[ ]" };
        println!("{} {} {}", item.id, status, item.todo);
    }
}

pub fn toggle_done(items: &mut Vec<Item>, id : u64){
    // change the toggle -> need to specify id
    match items.binary_search_by_key(&id, |item| item.id) {
        Ok(index) => items[index].done = !items[index].done,
        Err(_) => println!("Not found"),
    }
}

pub fn remove(items: &mut Vec<Item>, id: u64){
    // remove from our list -> specify id
    match items.binary_search_by_key(&id, |item| item.id) {
        Ok(index) => items.remove(index),
        Err(_) => println!("Not found"),
    }
}

pub fn save(filename: String, filepath: String){
    // save the todo list that we have in Vector of Items
}

pub fn load(filepath: String){
    // load the file (.todo)
}
