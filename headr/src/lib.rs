use clap::{App};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Edgar Morales <nekrum@gmail.com")
        .about("Rust head")
        .get_matrches();
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap_or_default(),
        lines: matches.value_of_lossy("lines").unwrap_or_default().parse()?,
        bytes: matches.value_of_lossy("bytes").map(|b| b.parse()).transpose()?,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    unimplemented!()
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert!(res.unwrap() == 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
