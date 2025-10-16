use clap::Parser;
use playerctl::PlayerCtl;
use std::{thread, time};
use substring::Substring;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Length of the outputed value
    #[arg(short, long, default_value_t = 20)]
    length: usize,

    /// ms of time to wait between outputs
    #[arg(short, long, default_value_t = 300)]
    delay: u64,

    /// ms of time to wait before scrolling
    #[arg(short, long, default_value_t = 300)]
    wait_time: u64,

    /// custom icon
    #[arg(short, long, default_value_t = String::from("\u{f1bc} "))]
    custom_icon: String,
}

fn main() -> ! {
    let cli = Cli::parse();
    let mut string: String = "this is a test".to_string();
    let mut index = 0;
    loop {
        string = update_string(&string);
        print_string(&string, index, cli.length, &cli.custom_icon);
        thread::sleep(time::Duration::from_millis(cli.delay));
        index += 1;
        if index > string.len() + 3 {
            thread::sleep(time::Duration::from_millis(cli.wait_time));
            index = 1;
        }
    }
}

fn update_string(last_string: &str) -> String {
    let metadata = PlayerCtl::metadata();
    let new_string = metadata.artist + " - " + &metadata.title;
    //println!("{}", &new_string);
    if last_string != new_string {
        new_string
    } else {
        last_string.to_string()
    }
}

fn print_string(string: &str, index: usize, length: usize, custom_icon: &str) {
    if string.len() > length {
        let scroll_substring = string.to_owned() + " | " + string;
        println!(
            "{custom_icon}{}",
            &scroll_substring.substring(index, index + length)
        )
    } else {
        println!("{custom_icon}{}", &string);
    }
}
