pub mod lib;
use lib::{get_config, get_json, QueryType};

fn main() {
    let config = get_config().unwrap();

    let search_word: String = match config{
        QueryType::Dictionary(word) => word,
        QueryType::Thesaurus(word) => word
    };

    println!("{}", search_word);
    let parsed_stuff = get_json(search_word).unwrap();
    println!("{:#?}", parsed_stuff);
}
