use clap::Parser;
use playerctl::{PlayerCtl, TrackMetadata};
use std::{thread::sleep, time::Duration};
use substring::Substring;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Length of the output value
    #[arg(short, long, default_value_t = 20)]
    length: usize,

    /// Time to wait between outputs in ms
    #[arg(short, long, default_value_t = 300)]
    delay: u64,

    /// Time to wait before scrolling in ms
    #[arg(short, long, default_value_t = 300)]
    wait_time: u64,

    /// Custom icon
    #[arg(short, long, default_value_t = String::from("\u{f1bc} "))]
    custom_icon: String,
}

fn main() -> ! {
    let cli = Cli::parse();
    let mut string: String = String::new();
    let mut index = 0;
    loop {
        string = update_string(&string);
        print_string(&string, index, cli.length, &cli.custom_icon);
        sleep(Duration::from_millis(cli.delay));
        index += 1;
        if index > string.len() + 3 {
            sleep(Duration::from_millis(cli.wait_time));
            index = 1;
        }
    }
}

fn update_string(last_string: &str) -> String {
    let TrackMetadata { artist, title, .. } = PlayerCtl::metadata();
    let new_string = format!("{artist} - {title}");
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
