use colored::Colorize;
use clap::{command, Arg, Command};
use serde::Deserialize;
// use serde_json;

#[derive(Debug)]
// This enum tells the main function which 
// print method (dictionary or thesaurus) needs to be used
// get_config returns this
pub enum QueryType{
    Dictionary(String),
    Thesaurus(String),
}

impl QueryType{
    pub fn new()-> Result<QueryType, Box<dyn std::error::Error>>{
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
    
        let query_type: QueryType;
    
    
        if let Some(def_matches) = matches.subcommand_matches("def") {
            if let Some(def_word) = def_matches.get_one::<String>("def_word") {
                query_type = QueryType::Dictionary(def_word.to_string());
            } else {
                return Err("Invalid word entered for def".into());
            }
        } else if let Some(thes_matches) = matches.subcommand_matches("thes") {
            if let Some(thes_word) = thes_matches.get_one::<String>("thes_word") {
                query_type = QueryType::Thesaurus(thes_word.to_string());
            } else {
                return Err("Invalid word entered for thes".into());
            }
        } else {
            return Err("No subcommand was used".into());
        }
        Ok(query_type)
    }
}

// these are the structs the api response deserializes into
// '->' means 'contains'
// Wordinfo -> Singlemeaning -> Definition
#[derive(Debug, Deserialize, Clone)]
pub struct WordInfo{
    word: String,
    meanings: Vec<SingleMeaning>,
}

impl WordInfo{
    pub fn dictionary(&self){
        for meaning in self.meanings.iter(){
            println!("{}-----------{}", self.word.to_uppercase().green(), meaning.part_of_speech.to_ascii_uppercase().green());
            for def_obj in meaning.definitions.iter(){
                println!("{}", def_obj.definition.bold());
                println!("Example: {}\n", def_obj.example.clone().unwrap_or("N/A".to_string()).blue());
            }
        }
    }

    pub fn thesaurus(&self){
        let mut syn_list: String = String::new();
        let mut ant_list: String = String::new();
    
        for meaning in self.meanings.iter(){
            for syn in meaning.synonyms.iter(){
                syn_list.push_str(format!("{}, ", syn).as_str());
            }
    
            for ant in meaning.antonyms.iter(){
                ant_list.push_str(format!("{}, ", ant).as_str());
            }
        }
    
        println!("Synonyms and antonyms for {}", self.word.to_uppercase().green());
        println!("Synonyms: {}", syn_list);
        println!("Antonyms: {}", ant_list);
    }
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

pub fn get_json(search_word: &String) -> Result<WordInfo, Box<dyn std::error::Error>>{
    let url: String = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", search_word);

    let res = ureq::get(&url).call()?.into_string()?;

    let word_info: Vec<WordInfo> = serde_json::from_str(&res)?;

    Ok(word_info.first().unwrap().clone())
}