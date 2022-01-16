use std::path::Path; 
use std::fs::File; 
use std::io::Write;
use std::io::{ BufRead, BufReader };

fn parse_md_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Attempting to parse {}...", _filename);

    // Create a path var from the filename
    // The call to Path::new() creates a new Path object
    let input_filename = Path::new(_filename);
    let file = File::open(input_filename).expect("[ ERROR ] Failed to open file");  

    let mut p: bool = false; // paragraph tags
    let mut h1: bool = false; // h1 tags

    let mut tokens: Vec<String> = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();
        let mut char0: Vec<char> = line_contents.chars().take(1).collect();
        let mut output_line = String::new();
        match char0.pop() {
            Some('#') => {
                if p { 
                    p  = false;
                    output_line.push_str("</p>\n");
                }
                if h1 { 
                    h1  = false;
                    output_line.push_str("</h1>\n");
                }
                h1 = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_contents[2..]);
            },
            _ => {
                if !p {
                    p  = true; 
                    output_line.push_str("<p>");
                }
                output_line.push_str(&line_contents);
            }
        }
        if p { 
            p = false; 
            output_line.push_str("</p>\n");
        }
        if h1 { 
            h1 = false; 
            output_line.push_str("</h1>\n");
        }
        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    for token in &tokens {
        println!("{}", token);
    }

    let mut output_filename = String::from(&_filename[.._filename.len() - 3]);
    output_filename.push_str(".html");

    let mut output_file = File::create(output_filename)
        .expect("[ ERROR ] Could not create output html file");

    for token in &tokens {
        output_file
            .write_all(token.as_bytes())
            .expect("[ ERROR ] Could not write"); 
    }

    println!("[ INFO ] Parsing complete");

    
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner(); 
    println!(
        "Created by: {}\nHomepage: {}\nUsage: tinymd <file.md>\n", 
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), "); 
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn usage() { 
    print_long_banner();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_md_file(&args[1]), 
        _ => usage(),
    }
}
