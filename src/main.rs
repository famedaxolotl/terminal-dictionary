pub mod lib;
use lib::get_config;

fn main() {
    get_config().unwrap();
}
