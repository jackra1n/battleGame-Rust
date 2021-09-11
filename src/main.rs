use std::io;
use text_io::read;

use crate::warrior::Warrior;

mod ui_maker;
mod warrior;

fn main() {
    ui_maker::print_menu("HomeScreen");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    print_menu_choose();
}

fn print_menu_choose() {
    ui_maker::print_menu("MainMenu");
    menu_choose(read!());
}

fn menu_choose(option: i32) {
    match option {
        1 => start_game(),
        2 => ui_maker::print_menu("Instructions"),
        3 => ui_maker::print_menu("Credits"),
        4 => ui_maker::print_menu("ClassStats"),
        _ => print_menu_choose(),
    }
}

fn check_names(name1: String, mut name2: String) {
    if name1 == name2 {
        println!("The names are the same. The second name will have '2' addded at the end.");
        name2.push_str("2"); 
    }
}

fn start_game() {
    let mut name = String::new();

    println!("Name the first warrior: ");
    io::stdin().read_line( &mut name).expect("Failed to read line");
    let mut warrior1 = Warrior::new(&name, 1, 1, 1);
    ui_maker::class_chooser();
    warrior1.choose_class(read!());

    println!("Name the second warrior: ");
    io::stdin().read_line( &mut name).expect("Failed to read line");
    let mut warrior2 = Warrior::new(&name, 1, 1, 1);
    ui_maker::class_chooser();
    warrior2.choose_class(read!());

    check_names(warrior1.name, warrior2.name);
    start_game();
}

fn start_fight(warrior1: Warrior, warrior2: Warrior) {
    while warrior1.health > 0 || warrior2.health > 0 {
        if check_for_winner(warrior1, warrior2) {
            break;
        }
        if check_for_winner(warrior2, warrior1) {
            break;
        }
    }
}

fn check_for_winner(warrior1: Warrior, warrior2: Warrior) -> bool {
    
}