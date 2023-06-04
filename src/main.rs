use clap::{App, Arg};
use std::{
    fs,
    io::{self, Read},
};

fn read_piped_value() -> String {
    let mut piped_value = String::new();
    io::stdin()
        .read_to_string(&mut piped_value)
        .expect("Failed to read piped value");
    piped_value.trim().to_string()
}

fn main() {
    let matches = App::new("Fast WC")
        .version("1.0")
        .author("Hezarfendede")
        .arg(Arg::with_name("input").index(1).help("Input file name"))
        .arg(
            Arg::with_name("count")
                .short("c")
                .help("Get number of bytes"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .help("Get number of lines"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .help("Get number of words"),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .help("Get numbers of chars"),
        )
        .get_matches();

    let file_path = matches.value_of("input");
    let contents = match file_path {
        Some(path) => fs::read_to_string(path).unwrap_or_else(|_| read_piped_value()),
        None => read_piped_value(),
    };

    let counting_options = vec![
        ("count", contents.capacity()),
        ("lines", contents.lines().count()),
        ("words", contents.split_whitespace().count()),
        ("chars", contents.chars().count()),
    ];

    let selected_option = counting_options
        .iter()
        .find(|(option, _)| matches.is_present(option));

    if let Some((_, count)) = selected_option {
        println!("{} {}       ", count, file_path.unwrap_or(""),);
    } else {
        println!(
            "{} {} {} {}",
            counting_options[1].1,
            counting_options[2].1,
            counting_options[0].1,
            file_path.unwrap_or(""),
        );
    }
}
