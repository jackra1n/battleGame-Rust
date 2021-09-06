use std::fs;


// fn get_text_menu(text_menu: &str) -> Result<String, std::io::Error> {
fn get_text_menu(text_menu: &str) -> String {
    let mut path = "/resources/text_menus/".to_owned();
    path.push_str(text_menu);
    path.push_str(".txt");
    // println!("result: {}", main_path);
    return path;
    // return fs::read_to_string();
}

fn main() {
    // print_main_menu();
    get_text_menu("MainMenu");
}

fn print_main_menu() {
    println!("Main");
}
