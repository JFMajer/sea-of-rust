use clap::{App, Arg};

fn main() {
    let _matches = App::new("rechor")
      .version("0.1.0")
      .about("Echo in rust")
      .get_matches();
}
