use std::fs::File;
use std::io::prelude::*;
use std::io;
use text_io::read;


fn get_menu(menu: &str) -> String {
    let mut path = "resources/text_menus/".to_owned();
    path.push_str(menu);
    path.push_str(".txt");
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    return contents;
}

fn print_menu(menu: &str) {
    print!("{}", get_menu(menu));
}

fn print_menu_choose() {
    print!("{}", get_menu("MainMenu"));
    let input: i32 = read!();
    menu_choose(input)
}

fn menu_choose(option: i32) {
    if option == 1 {
        start_game();
    }
    else if option == 2 {
        print_menu("Instructions");
    }
    else if option == 3 {
        print_menu("Credits");
    }
    else if option == 4 {
        print_menu("ClassStats")
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    print!("it should wait before this");
    print_menu_choose();
}

fn start_game() {
    print!("Name the first warrior: ");
}

fn main() {
    print_menu("HomeScreen");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    print_menu_choose();
}