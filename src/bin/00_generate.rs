// used to scaffold scripts for each day & and pull down the input


use std::io::Write;
use std::{fs, io};

fn main() {
    // ask user for day number to generate
    let mut input = String::new();

    print!("Enter day number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let day_num: i32 = input.trim().parse().expect("Please enter a number!");

    // generate paths for script & input files we will write
    let script_path = format!("src/bin/{day_num:02}a.rs");
    let input_path = format!("inputs/{day_num:02}.txt");

    // check if either file already exists
    if fs::metadata(&script_path).is_ok() || fs::metadata(&input_path).is_ok() {
        println!("Script or input already exists!");
        return;
    }

    // load template & replace strings as needed
    let script_code = fs::read_to_string("src/bin/00_template.rs")
        .unwrap()
        .replace("00", &format!("{day_num:02}"));

    // write both our a & b parts
    fs::write(&script_path, script_code).unwrap();

    // download input from AoC website
    let input_contents = download_input_from_aoc_api(day_num);
    fs::write(input_path, input_contents).expect("Failed to write input file");

    println!("Generated script & input files!");
    println!("To run: cargo run --bin {day_num:02}a");

    std::process::Command::new("idea")
        .arg(script_path)
        .spawn()
        .expect("Failed to open script in IDE");
}

fn download_input_from_aoc_api(day_num: i32) -> String {
    let input_url = format!("https://adventofcode.com/2023/day/{day_num}/input");

    // load session token from env
    let session_token = std::env::var("AOC_SESSION_TOKEN").expect("AOC_SESSION_TOKEN env not set");

    let client = reqwest::blocking::Client::new();

    // set request headers (auth cookie)
    let mut headers = reqwest::header::HeaderMap::new();
    let cookie_value = format!("session={}", session_token);
    headers.insert(
        "Cookie",
        reqwest::header::HeaderValue::from_str(&cookie_value).unwrap(),
    );

    // Build the request with custom headers
    let response = client
        .get(input_url)
        .headers(headers)
        .send()
        .expect("Failed to send request");

    // Check if the request was successful
    if !response.status().is_success() {
        panic!("Failed to fetch the file: {}", response.status());
    }

    

    response.text().expect("Failed to read response text")
}
