// TODO-LIST

use clearscreen;
use std::{
    io::{self, stdout, Write},
    time,
};
use uuid::Uuid;
use chrono::{DateTime, Local, NaiveDate}

#[derive(Debug)]
struct TodoItem {
    id: Uuid,
    done: bool,
    description: String,
    title: String,
    created_at: time::SystemTime,
    expires_at: Option<NaiveDate>,
}

fn print_menu() {
    println!("*************************************");
    println!("*** Welcome to the  todo list app ***");
    println!("*************************************");
    println!("1. Add a todo item");
    println!("2. Remove a todo item");
    println!("3. Show all todo items with");
    println!("0. Exit");
}

fn add_todo_item() -> TodoItem {
    println!("Enter the title of the todo item: ");
    let id = Uuid::new_v4();
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    println!("Enter the description of the todo item: ");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
    let mut expires_at = String::new();
    println!("Enter the expiration date (YYYY-MM-DD): ");
    io::stdin()
        .read_line(&mut expires_at)
        .expect("Failed to read line");

    TodoItem {
        id,
        done: false,
        title: title.trim().to_string(),
        description: description.trim().to_string(),
        created_at: time::SystemTime::now(),
        expires_at: NaiveDate::parse_from_str(&expires_at.trim().to_string(), "%Y-%m-%d"),
    }
}

fn clear_screen() {
    clearscreen::clear().expect("Failed to clear screen");
}
fn ask_for_input(message: &str) -> String {
    print!("{}", message);
    stdout().flush().unwrap();
    let mut input_val = String::new();
    io::stdin()
        .read_line(&mut input_val)
        .expect("Failed to read line");
    input_val
}

fn main() {
    let mut items: Vec<TodoItem> = Vec::new();
    clear_screen();
    print_menu();
    let choice_text = "Enter your choice: ";
    let mut input_val = ask_for_input(choice_text);
    loop {
        match input_val.trim() {
            "0" => {
                println!("Exiting...");
                break;
            }
            "1" => {
                clear_screen();
                print_menu();
                let todo_item = add_todo_item();
                items.push(todo_item);
                clear_screen();
                print_menu();
                input_val = ask_for_input(choice_text);
            }
            "2" => {
                clear_screen();
                print_menu();
                let total_items = items.len();
                println!("Remove a todo item");
                println!("");
                println!("Total items: {}", total_items);
                println!("");
                if total_items == 0 {
                    println!("No items found");
                }
                items.iter().enumerate().for_each(|(idx, item)| {
                    println!("#{} | [{}] {}", idx, item.title, item.description);
                });
                println!("");
                let index_to_remove =
                    ask_for_input("Enter the number of the todo item to remove: ");
                let index_to_remove: usize = index_to_remove.trim().parse().unwrap();
                items.remove(index_to_remove);
                ask_for_input(choice_text);
            }
            "3" => {
                clear_screen();
                print_menu();
                let total_items = items.len();
                println!("");
                println!("Total items: {}", total_items);
                println!("");
                if total_items == 0 {
                    println!("No items found");
                }
                items.iter().enumerate().for_each(|(idx, item)| {
                    println!(
                        "#{} | [{}] | {} | {}",
                        idx,
                        item.title,
                        item.description,
                        item.date_time
                        .
                    );
                });
                println!("");
                input_val = ask_for_input(choice_text);
            }
            _ => println!("Invalid choice"),
        }
        stdout().flush().unwrap();
    }
}
