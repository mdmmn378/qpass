use base64::prelude::*;
use clap::{ArgAction, Command, Error};
use colored::*;
use rand::Rng;

fn generate_random_password(length: u16) -> String {
    let mut password = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        let random_char: char = rng.gen_range(33..127).into();
        password.push(random_char);
    }
    return password;
}

fn convert_to_base64(password: String) -> String {
    return BASE64_STANDARD.encode(password.as_bytes());
}

fn generate(val: &clap::ArgMatches) -> Result<(), Error> {
    let _from_opt = val.get_one::<String>("from");
    let mut _from: String = "".to_string();
    if !_from_opt.is_none() {
        _from = _from_opt.unwrap().clone();
    }
    let _length = val.get_one::<u16>("length").unwrap().clone();
    if _from.len() > 0 {
        println!("{}", convert_to_base64(_from).green());
        return Ok(());
    } else {
        let password = generate_random_password(_length).blue();
        println!("{}", password);
    }

    return Ok(());
}

fn main() -> Result<(), Error> {
    let cmd = Command::new("qpass")
        .about("Generate a password")
        .bin_name("qpass")
        .arg(
            clap::Arg::new("from")
                .help("Generate a password from a string")
                .long("from")
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            clap::Arg::new("length")
                .long("length")
                .help("Length of the password")
                .required(false)
                .value_parser(clap::value_parser!(u16).range(12..128))
                .action(ArgAction::Set)
                .default_value("16"),
        );
    let matches = cmd.get_matches();
    match matches.args_present() {
        true => generate(&matches),
        false => Ok(()),
    }?;
    return Ok(());
}
