use reqwest::{self, blocking::Client};
use std::{
    error::Error,
    process::{Command, Stdio},
    time::Duration,
};

fn add_protocol(url: String) -> String {
    let protocol: String = "https://".to_owned();
    if url.contains(&protocol) {
        url
    } else {
        protocol + &url
    }
}

fn fetch_response(url: String, mut tries: u64, waiting_period: u64) -> Result<(), Box<dyn Error>> {
    let final_delay = tries;

    let url: reqwest::Url = match reqwest::Url::parse(&url) {
        Ok(val) => val,
        Err(_) => {
            println!("Failed to parse url!");
            std::process::exit(0);
        }
    };

    let mut first_run: bool = true;

    loop {
        if first_run {
        } else {
            tries -= 1;
            if tries == 0 {
                println!("Tries completed with delay of {waiting_period}s for each try!");
                break;
            }
            println!(
                "Retrying {} times with delay of {waiting_period} seconds..",
                final_delay - tries + 1
            );
            std::thread::sleep(Duration::from_secs(waiting_period));
        }
        println!("Trying to Fetch response from {url}...");

        let client = Client::builder()
            .timeout(Duration::from_secs(waiting_period))
            .build()?;

        let response = match client.get(url.clone()).send() {
            Ok(response) => response,
            Err(_) => {
                println!("Failed to get any response");
                first_run = false;
                continue;
                // std::process::exit(0);
            }
        };

        println!("Checking response status...");
        if response.status() != 200 {
            println!("Error: {}", response.status());
            first_run = false;

            continue;
        } else {
            println!("Bingo Its Working now!");
            println!("{} is ready to use!", &url);
            // LINUX
            if cfg!(target_os = "linux") {
                println!("Trying to Open Default Browser..");
                Command::new("xdg-open")
                    .arg(url.to_string())
                    .stdout(Stdio::null()) // Hide standard output
                    .stderr(Stdio::null()) // Hide error output
                    .spawn()?;
            }

            break;
        }
    }

    Ok(())
}

/// Some Domains for ease of use
fn return_sites(arg: &str) -> String {
    let url: String = add_protocol(arg.to_owned());
    match arg {
        "google" => "https://google.com".to_owned(),
        "aur" => "https://aur.archlinux.org/".to_owned(),
        "arch" => "https://archlinux.org/".to_owned(),
        "cht" => "https://cht.sh/".to_owned(),
        _ => url,
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let mut waiting_period: u64 = 30;
    let mut url_arg: String = String::new();
    let url_ref: &mut String = &mut url_arg;
    let mut default_tries: u64 = 5;

    if args.len() == 3 {
        let temp: String = return_sites(args[1].trim());
        *url_ref = temp;

        let tries_arg: &str = args[2].trim();
        match tries_arg.parse::<u64>() {
            Ok(val) => default_tries = val,
            Err(_) => {
                println!("Invalid time, it has be u64!");
                std::process::exit(0);
            }
        }
    } else if args.len() == 4 {
        let temp: String = return_sites(args[1].trim());
        *url_ref = temp;

        let time: String = args[3].clone();
        match time.trim().parse::<u64>() {
            Ok(val) => {
                waiting_period = val;
            }
            Err(_) => {
                println!("Invalid time, please provide in u64 as seconds!");
                std::process::exit(0);
            }
        }
        let tries_arg: &str = args[2].trim();
        match tries_arg.parse::<u64>() {
            Ok(val) => default_tries = val,
            Err(_) => {
                println!("Invalid input, it has be u64!");
            }
        }
    } else if args.len() == 2 {
        if args[1] == "help" {
            println!("Usage:\ncheck [url] [retries] [delay in secs]\ncheck google.com 10 60\ncheck [url]\ncheck [url] [retires]"
            );
            return Ok(());
        }
        let temp: String = return_sites(args[1].trim());
        *url_ref = temp;
    } else {
        println!("No url or  waiting period is provided!, terminating...");
        println!("Help\ncheck [url] [retries] [delay in secs]\nUsage:\ncheck google.com 10 60\ncheck [url]\ncheck [url] [retires]"
            );
        return Ok(());
    }
    if default_tries == 0 {
        println!("Can't be 0 times, reset back to {}", 5);
        default_tries = 5;
    }

    println!("Will try for {default_tries} times with every {waiting_period}s delay");
    fetch_response(url_arg.clone(), default_tries, waiting_period)?;
    Ok(())
}
