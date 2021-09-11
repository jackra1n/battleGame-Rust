use std::fs::File;
use std::io::prelude::*;

fn get_menu(menu: &str) -> String {
    let mut path = "resources/text_menus/".to_owned();
    path.push_str(menu);
    path.push_str(".txt");
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    return contents;
}

pub fn print_menu(menu: &str) {
    print!("{}", get_menu(menu));
}

pub fn class_chooser() {
    print_menu("ClassChooser")
}
