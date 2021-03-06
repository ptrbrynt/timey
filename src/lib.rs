use chrono::{format::ParseError, prelude::*};
use clap::{App, Arg, SubCommand};
use clipboard::{ClipboardContext, ClipboardProvider};

pub fn run() -> Result<(), &'static str> {
    let matches = App::new("timey")
        .version("1.0")
        .author("Peter Bryant <peter@ptrbrynt.com>")
        .about("Simple date-time parse/formatter")
        .subcommand(
            SubCommand::with_name("parse")
                .about("Converts a date-time string into a timestamp")
                .arg(
                    Arg::with_name("copy")
                        .short("c")
                        .long("copy")
                        .help("Copy the result to the clipboard"),
                )
                .arg(
                    Arg::with_name("format")
                        .short("f")
                        .long("format")
                        .help("The date-time format of the input string")
                        .required(true)
                        .value_name("FORMAT")
                        .default_value("%+")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("milliseconds")
                        .short("m")
                        .long("millis")
                        .help("Return the timestamp in milliseconds rather than seconds"),
                )
                .arg(
                    Arg::with_name("input")
                        .required(true)
                        .takes_value(true)
                        .help("A string representing a date-time"),
                ),
        )
        .subcommand(
            SubCommand::with_name("format")
                .about("Formats a timestamp as a date-time string")
                .arg(
                    Arg::with_name("copy")
                        .short("c")
                        .long("copy")
                        .help("Copy the result to the clipboard"),
                )
                .arg(
                    Arg::with_name("milliseconds")
                        .short("m")
                        .long("millis")
                        .help("Parse the input as a timestamp in milliseconds"),
                )
                .arg(
                    Arg::with_name("format")
                        .short("f")
                        .long("format")
                        .value_name("FORMAT")
                        .help("The date-time format of the output string")
                        .required(true)
                        .default_value("%+")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("input")
                        .required(true)
                        .takes_value(true)
                        .help(
                        "An epoch timestamp in seconds (or in milliseconds if the -m flag is set)"
                    ),
                ),
        )
        .subcommand(
            SubCommand::with_name("now")
                .subcommand(
                    SubCommand::with_name("format")
                        .arg(
                            Arg::with_name("format")
                                .short("f")
                                .long("format")
                                .required(true)
                                .help("The date-time format in which to display the current date-time")
                                .takes_value(true)
                                .default_value("%+")
                                .value_name("FORMAT"),
                        )
                        .arg(
                            Arg::with_name("copy")
                                .short("c")
                                .long("copy")
                                .help("Copy the result to the clipboard"),
                        ),
            )
            .subcommand(
                SubCommand::with_name("display")
                .arg(
                    Arg::with_name("copy")
                        .short("c")
                        .long("copy")
                        .help("Copy the result to the clipboard"),
                )
                .arg(
                    Arg::with_name("milliseconds")
                        .short("m")
                        .long("millis")
                        .help("Display the current timestamp in milliseconds"),
                ),
            ),
        )
        .get_matches();

    let (subcommand, subcommand_matches) = matches.subcommand();

    let subcommand_matches = match subcommand_matches {
        Some(value) => value,
        None => {
            println!("Please specify a subcommand, or type 'timey -h' for help");
            return Ok(());
        }
    };

    if subcommand == "parse" {
        let input = subcommand_matches.value_of("input").unwrap();
        let fmt = subcommand_matches.value_of("format").unwrap();
        let millis = subcommand_matches.is_present("milliseconds");
        let copy = subcommand_matches.is_present("copy");
        let result: Result<i64, ParseError> = parse(input, fmt, millis, copy);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("There was an error parsing the input"),
        }
    } else if subcommand == "format" {
        let input: i64 = subcommand_matches
            .value_of("input")
            .unwrap()
            .parse()
            .unwrap();
        let fmt = subcommand_matches.value_of("format").unwrap();
        let millis = subcommand_matches.is_present("milliseconds");
        let copy = subcommand_matches.is_present("copy");
        let result = format(input, fmt, millis, copy);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("There was an error formatting the input"),
        }
    } else if subcommand == "now" {
        let (now_sub, now_matches) = subcommand_matches.subcommand();
        let now_matches = now_matches.unwrap();
        if now_sub == "format" {
            let fmt = now_matches.value_of("format").unwrap();
            let copy = now_matches.is_present("copy");
            let result = format_now(fmt, copy);
            match result {
                Ok(_) => Ok(()),
                Err(_) => Err("An error occurred"),
            }
        } else if now_sub == "display" {
            let copy = now_matches.is_present("copy");
            let millis = now_matches.is_present("milliseconds");
            display_now(millis, copy);
            Ok(())
        } else {
            Err("Unrecognised subcommand")
        }
    } else {
        Err("Unrecognised subcommand")
    }
}

fn parse(input: &str, format: &str, millis: bool, copy: bool) -> Result<i64, ParseError> {
    let parsed = DateTime::parse_from_str(input, format);

    match parsed {
        Ok(value) => {
            let timestamp = if millis {
                value.timestamp_millis()
            } else {
                value.timestamp()
            };
            if copy {
                copy_to_clipboard(&timestamp.to_string());
            }
            println!("{}", timestamp);
            Ok(timestamp)
        }
        Err(err) => {
            eprintln!("{}", err);
            Err(err)
        }
    }
}

fn format(input: i64, format: &str, millis: bool, copy: bool) -> Result<String, &'static str> {
    let dt = if millis {
        Utc.timestamp_millis(input)
    } else {
        Utc.timestamp(input, 0)
    };

    let formatted = dt.format(format).to_string();

    println!("{}", formatted);
    if copy {
        copy_to_clipboard(&formatted);
    }
    Ok(formatted)
}

fn copy_to_clipboard(data: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(data.to_owned()).unwrap();
}

fn format_now(format: &str, copy: bool) -> Result<String, &'static str> {
    let dt = Utc::now();

    let formatted = dt.format(format).to_string();

    println!("{}", formatted);
    if copy {
        copy_to_clipboard(&formatted);
    }
    Ok(formatted)
}

fn display_now(millis: bool, copy: bool) -> i64 {
    let dt = Utc::now();
    let timestamp = if millis {
        dt.timestamp_millis()
    } else {
        dt.timestamp()
    };

    println!("{}", timestamp);
    if copy {
        copy_to_clipboard(&timestamp.to_string());
    }

    timestamp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_with_default_format() {
        let result = parse("2019-03-06T14:40:20+00:00", "%+", false, false).unwrap();
        let expected: i64 = 1_551_883_220;
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_with_custom_format() {
        let result = parse(
            "2019-03-06 14:40:20 +0000",
            "%Y-%m-%d %H:%M:%S %z",
            false,
            false,
        )
        .unwrap();
        let expected: i64 = 1_551_883_220;
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_with_default_format_millis() {
        let result = parse("2019-03-06T14:40:20+00:00", "%+", true, false).unwrap();
        let expected: i64 = 1_551_883_220_000;
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_with_custom_format_millis() {
        let result = parse(
            "2019-03-06 14:40:20 +0000",
            "%Y-%m-%d %H:%M:%S %z",
            true,
            false,
        )
        .unwrap();
        let expected: i64 = 1_551_883_220_000;
        assert_eq!(expected, result);
    }

    #[test]
    fn format_with_default_format() {
        let result = format(1_551_883_220, "%+", false, false).unwrap();
        assert_eq!("2019-03-06T14:40:20+00:00", result);
    }

    #[test]
    fn format_with_custom_format() {
        let result = format(1_551_883_220, "%Y-%m-%d %H:%M:%S %z", false, false).unwrap();
        assert_eq!("2019-03-06 14:40:20 +0000", result);
    }

    #[test]
    fn format_with_default_format_millis() {
        let result = format(1_551_883_220_000, "%+", true, false).unwrap();
        assert_eq!("2019-03-06T14:40:20+00:00", result);
    }

    #[test]
    fn format_with_custom_format_millis() {
        let result = format(1_551_883_220_000, "%Y-%m-%d %H:%M:%S %z", true, false).unwrap();
        assert_eq!("2019-03-06 14:40:20 +0000", result);
    }
}
