use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut urls = HashMap::new();
    loop {
        print!("Enter URL to shorten (or 'exit'): ");
        io::stdout().flush().unwrap();
        let mut url = String::new();
        io::stdin().read_line(&mut url).unwrap();
        let url = url.trim();
        if url == "exit" { break; }
        let code = format!("u{}", urls.len() + 1);
        urls.insert(code.clone(), url.to_string());
        println!("Shortened: {} -> {}", code, url);
    }
}
