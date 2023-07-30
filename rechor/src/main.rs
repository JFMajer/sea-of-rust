use clap::{App, Arg};

fn main() {
    let matches = App::new("rechor")
        .version("0.1.0")
        .about("Echo in rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Don't print newline")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("prints_a_frog")
                .short("f")
                .help("Prints a frog")
        )
        .get_matches();

   // println!("{:#?}", matches);

   let text = matches.values_of_lossy("text").unwrap();
   let omit_newline = matches.is_present("omit_newline");
   let frog = matches.is_present("prints_a_frog");
   let frog_emoji = '\u{1F438}';
   let ending = if omit_newline { "" } else { "\n" };
   
    // construct the output string
    let mut output = String::new();

    if frog {
        output += &text.join(" ");
        output += &format!(" {}", frog_emoji);
        output += ending;
    } else {
        output += &text.join(" ");
        output += ending;
    }


    print!("{}", output);
   

}
