pub mod lib;
use lib::{get_config, get_json};

fn main() {
    let search_word = get_config().unwrap();
    println!("{}", search_word);
    let parsed_stuff = get_json(search_word).unwrap();
    println!("{:#?}", parsed_stuff);
}
