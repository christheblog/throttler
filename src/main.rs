extern crate clap;

use clap::{App, Arg};

use std::io::BufRead;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {

    let args = App::new("throttler")
        .version("0.1.0")
        .author("christheblog")
        .about("Throttles stdout")
        .arg(
            Arg::with_name("rate")
                .long("rate")
                .short("r")
                .help("Sets the output rate")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("no-warning")
                .long("no-warning")
                .short("n")
                .help("Removes the skipped lines warning")
                .required(false),
        )
        .get_matches();

    // Reading arguments
    let rate: Rate = args.value_of("rate").and_then(|x| parse_rate(x)).unwrap();
    let display_warning: bool;
    if args.is_present("no-warning") {
        display_warning = false;
    } else {
        display_warning = true;
    }

    // Reading Stdin
    let mut next = timestamp_ms() + rate.to_millis();
    let mut counter = 0;
    let mut skipped = 0;
    let stdin = io::stdin();
    for line_result in stdin.lock().lines() {
        let now = timestamp_ms();
        // Resetting parameters if needed
        if now >= next {
            next = now + rate.to_millis();
            counter = 0;
            if display_warning && skipped > 0 {
                println!("--- Skipped {} line(s) ---", skipped);
                skipped = 0;
            }
        }
        // Checking if we need to print the line or not
        if counter < rate.value() && now <= next {
            let line = line_result.unwrap();
            counter += 1;
            println!("{}", &line);
        } else {
            skipped += 1;
        }
    }
    // Displaying last skipped lines
    if display_warning && skipped > 0 {
        println!("--- Skipped {} line(s) ---", skipped);
    }
}

fn timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error when getting timestamp")
        .as_millis()
}


// Rate conversion + parsing

#[derive(PartialEq, Eq)]
enum Rate {
    Millis(u128),
    Second(u128),
    Minute(u128),
    Hour(u128),
    Day(u128),
}

impl Rate {

    fn to_millis(&self) -> u128 {
        use Rate::*;
        match self {
            Millis(r) => *r,
            Second(r) => r * 1000,
            Minute(r) => r * 1000 * 60,
            Hour(r) => r * 1000 * 3600,
            Day(r) => r * 1000 * 3600 * 24,
        }
    }

    fn value(&self) -> u128 {
        use Rate::*;
        match self {
            Millis(r) => *r,
            Second(r) => *r,
            Minute(r) => *r,
            Hour(r) => *r,
            Day(r) => *r,
        }
    }
}


fn parse_rate(str: &str) -> Option<Rate> {
    use Rate::*;
    if str.ends_with("/ms") {
        let mut rate = str.to_string();
        rate.truncate(str.len() - 3);
        rate.parse().ok().map(|x| Millis(x))
    } else if str.ends_with("/s") {
        let mut rate = str.to_string();
        rate.truncate(str.len() - 2);
        rate.parse().ok().map(|x| Second(x))
    } else if str.ends_with("/min") {
        let mut rate = str.to_string();
        rate.truncate(str.len() - 4);
        rate.parse().ok().map(|x| Minute(x))
    } else if str.ends_with("/h") {
        let mut rate = str.to_string();
        rate.truncate(str.len() - 2);
        rate.parse().ok().map(|x| Hour(x))
    } else if str.ends_with("/day") {
        let mut rate = str.to_string();
        rate.truncate(str.len() - 4);
        rate.parse().ok().map(|x| Day(x))
    } else {
        let rate = str.to_string();
        rate.parse().ok().map(|x| Second(x))
    }
}


// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rate_per_millis() {
        assert![parse_rate("12/ms") == Some(Rate::Millis(12))]
    }

    #[test]
    fn test_parse_rate_per_second() {
        assert![parse_rate("40/s") == Some(Rate::Second(40))]
    }

    #[test]
    fn test_parse_rate_per_minute() {
        assert![parse_rate("22/min") == Some(Rate::Minute(22))]
    }

    #[test]
    fn test_parse_rate_per_hour() {
        assert![parse_rate("47/h") == Some(Rate::Hour(47))]
    }

    #[test]
    fn test_parse_rate_per_day() {
        assert![parse_rate("725/day") == Some(Rate::Day(725))]
    }

    #[test]
    fn test_parse_rate_no_unit() {
        assert![parse_rate("25") == Some(Rate::Second(25))]
    }
}
