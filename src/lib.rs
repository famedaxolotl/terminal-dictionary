use clap::{command, Arg, Command};
use serde::{Deserialize};
use serde_json;
// pub trait ResType{
//     fn print(&self) -> Result<(), Box<dyn std::error::Error>>;
// }



pub fn get_config()-> Result<String, Box<dyn std::error::Error>> {
    let matches = command!()
    .about("Simple Dictionary on the terminal.")
    .version("0.1.0")
    .author("Axolotl rishavghosh2007@gmail.com")
    .subcommand(
        Command::new("def")
        .about("search dictionary")
        .arg(
            Arg::new("def_word")
            .help("word to search in dictionary")
            .required(true)
            .index(1)
        )
    )
    .subcommand(
        Command::new("thes")
        .about("search thesaurus")
        .arg(
            Arg::new("thes_word")
            .help("word to search in thesaurus")
            .required(true)
            .index(1)
        )
    ).get_matches();

    if let Some(def_matches) = matches.subcommand_matches("def") {
        if let Some(def_word) = def_matches.get_one::<String>("def_word") {
            Ok(def_word.clone())
        }else {
            Err("Invalid word entered".into())
        }
    }else{
        Err("Soething went wrong".into())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct WordInfo{
    word: String,
    meanings: Vec<SingleMeaning>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SingleMeaning{
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<Definition>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Definition{
    definition: String,
    example: Option<String>,
}

pub fn get_json(search_word: String) -> Result<WordInfo, Box<dyn std::error::Error>>{
    let url: String = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", search_word);

    let res = ureq::get(&url).call()?.into_string()?;

    let word_info: Vec<WordInfo> = serde_json::from_str(&res)?;

    Ok(word_info.get(0).unwrap().clone())
}