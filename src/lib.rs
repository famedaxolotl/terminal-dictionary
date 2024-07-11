use clap::{command, Arg, Command};
// pub trait ResType{
//     fn print(&self) -> Result<(), Box<dyn std::error::Error>>;
// }



pub fn get_config()-> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
    .about("Simple Dictionary on the terminal.")
    .version("0.1.0")
    .author("Axolotl rishavghosh2007@gmail.com")
    .subcommand(
        Command::new("def")
        .about("search dictionary")
        .arg(
            Arg::new("def-word")
            .help("word to search in dictionary")
            .required(true)
            .index(1)
        )
    )
    .subcommand(
        Command::new("thes")
        .about("search thesaurus")
        .arg(
            Arg::new("thes-word")
            .help("word to search in thesaurus")
            .required(true)
            .index(1)
        )
    ).get_matches();

    Ok(())
}