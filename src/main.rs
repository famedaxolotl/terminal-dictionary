pub mod lib;
use lib::{get_config, get_json, QueryType, get_dictionary, get_thesaurus};

fn main() {
    let config = get_config().unwrap();
    println!("{:?}", config);

match config{
        QueryType::Dictionary(word) => {
            let word_info = get_json(&word).unwrap();
            get_dictionary(word_info);
        },
        QueryType::Thesaurus(word) => {
            let word_info = get_json(&word).unwrap();
            get_thesaurus(word_info);
        }
    };
}
