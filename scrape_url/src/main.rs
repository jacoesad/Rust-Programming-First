use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!= 3 {
        println!("Usage: cargo run -- https://www.rust-lang.org rust.md");
        return;
    }
    
    let url = &args[1];
    let output = &args[2];
  
    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}