use std::fs::read_to_string;

fn get_headers_level(line: &str) -> u8 {
    let mut level = 0;
    line.chars().for_each(|c| {
        if c == '#' {
            level += 1;
        }
    });
    level
}

fn get_raw_title(line: &str) -> String {
    let mut raw_title = line.replace("#", "");
    if let Some(without_prefix) = raw_title.strip_prefix(" ") {
        raw_title = without_prefix.to_string();
    }
    raw_title
}

fn parse(content: Vec<String>) -> Vec<(u8, String)> {
    let mut titles = Vec::new();

    for line in content {
        if line.starts_with("#") {
            titles.push((get_headers_level(&line), get_raw_title(&line)));
        }
    }

    titles
}

fn generate_link(title: &str) -> String {
    title.replace(" ", "-").replace("'", "").to_lowercase()
}

fn generate_toc(titles: Vec<(u8, String)>) {
    for (level, title) in titles {
        for _ in 0..(level - 1) {
            print!("  ");
        }
        println!("- [{}](#{})", title, generate_link(&title));
    }
}

fn main() {
    // Get input file from args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./{} <MARKDOWN FILE>", args.get(0).unwrap());
        return;
    }

    // Read the file into a Vec<String> buffer
    let mut content = Vec::new();
    for line in read_to_string(args.get(1).unwrap()).unwrap().lines() {
        content.push(line.to_string());
    }

    // Parse the buffer
    let titles = parse(content);

    // Generate the table of contents
    generate_toc(titles);
}
