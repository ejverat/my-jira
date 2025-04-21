use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    // TODO: create database and navigator
    let db = Rc::new(JiraDatabase::new("data/db.json".to_owned()));
    let mut navigator = Navigator::new(db.clone());

    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        let current_page = navigator.get_current_page();
        if current_page.is_none() {
            break;
        }
        let current_page = current_page.unwrap();
        // 2. render page
        if let Err(error) = current_page.draw_page() {
            println!(
                "Error rendering page: {}\nPress any key to continue...",
                error
            );
            wait_for_key_press();
            continue;
        }
        // 3. get user input
        let input = get_user_input().trim().to_owned();
        // 4. pass input to page's input handler
        let action = current_page.handle_input(input.as_str());
        if let Err(error) = &action {
            println!(
                "Error handling input {}: {}\nPress any key to continue...",
                input, error
            );
            wait_for_key_press();
            continue;
        }

        // 5. if the page's input handler returns an action let the navigator process the action
        let action = action.unwrap();
        if let Some(action) = action {
            if let Err(error) = navigator.handle_action(action) {
                println!(
                    "Error handling this action: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
                continue;
            }
        }
    }
}
