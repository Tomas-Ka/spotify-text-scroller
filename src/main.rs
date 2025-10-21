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

    /// Icon
    #[arg(short, long, default_value_t = String::from("\u{f1bc}"))]
    icon: String,

    /// Disable icon
    #[arg(long, default_value_t = false)]
    disable_icon: bool,

    /// Disable author
    #[arg(long, default_value_t = false)]
    disable_author: bool,
}

fn main() -> ! {
    let cli = Cli::parse();
    let mut string: String = String::new();
    let mut index = 0;
    loop {
        string = update_string(&string, cli.disable_author);
        print_string(&string, index, cli.length, &cli.icon, cli.disable_icon);
        sleep(Duration::from_millis(cli.delay));
        index += 1;
        if index > string.len() + 3 {
            sleep(Duration::from_millis(cli.wait_time));
            index = 1;
        }
    }
}

fn update_string(last_string: &str, disable_author: bool) -> String {
    let TrackMetadata { artist, title, .. } = PlayerCtl::metadata();

    let new_string = if !disable_author {
        format!("{artist} - {title}")
    } else {
        title
    };

    if last_string != new_string {
        new_string
    } else {
        last_string.to_string()
    }
}

fn print_string(string: &str, index: usize, length: usize, icon: &str, disable_icon: bool) {
    let mut s = String::new();
    if !disable_icon {
        s.push_str(icon);
    }
    if string.len() > length {
        let scroll_substring = string.to_owned() + " | " + string;
        s.push_str(scroll_substring.substring(index, index + length));
    } else {
        s.push_str(string);
    }
    println!("{}", s);
}
