#![allow(special_module_name)]
pub mod lib;
use lib::{get_json, QueryType, get_dictionary, get_thesaurus};
use std::process;

fn main() {
    let config = QueryType::new().unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });
    
match config{
        QueryType::Dictionary(word) => {
            let word_info = get_json(&word).unwrap_or_else(|error| {
                eprintln!("{}", error);
                process::exit(1);
            });
            get_dictionary(word_info);
        },
        QueryType::Thesaurus(word) => {
            let word_info = get_json(&word).unwrap_or_else(|error| {
                eprintln!("{}", error);
                process::exit(1);
            });
            get_thesaurus(word_info);
        }
    };
}
