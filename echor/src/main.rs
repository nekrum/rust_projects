use clap::{App, Arg};


fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Edgar Morales <nekrum@gmail.com>")
        .about("Rust Echo")
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
            .help("Do not print newline")
            .takes_value(false),
        )
        .get_matches();
    let text = _matches.values_of_lossy("text").unwrap();
    let omit_newline = _matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
