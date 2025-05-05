use clap::Parser;
use playerctl::PlayerCtl;
use std::{thread, time};
use substring::Substring;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Length of the outputed value
    length: usize,
    /// ms of time to wait between outputs
    delay: u64,
    /// ms of time to wait before scrolling
    wait_time: u64,
}

fn main() -> ! {
    let cli = Cli::parse();
    let mut string: String = "this is a test".to_string();
    let mut index = 0;
    loop {
        string = update_string(&string);
        print_string(&string, index, cli.length);
        thread::sleep(time::Duration::from_millis(cli.delay));
        index = index + 1;
        if index > string.len() + 3 {
            thread::sleep(time::Duration::from_millis(cli.wait_time));
            index = 1;
        }
    }
}

fn update_string(last_string: &str) -> String {
    let metadata = PlayerCtl::metadata();
    let new_string = metadata.artist + " - " + &metadata.title;
    println!("{}", &new_string);
    if last_string != new_string {
        return new_string;
    } else {
        return last_string.to_string();
    }
}

fn print_string(string: &str, index: usize, length: usize) {
    if string.len() > length {
        let scroll_substring = string.to_owned() + " | " + string;
        println!("{}", &scroll_substring.substring(index, index + length))
    } else {
        println!("{}", &string);
    }
}
