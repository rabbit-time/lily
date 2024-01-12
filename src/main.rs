use rand::Rng;
use std::fmt;
use colored::Colorize;

fn main() {
    let quote = Quote::new();
    println!("{quote}");
}

struct Quote {
    text: String,
    position: usize,
    total: usize,
}

impl Quote {
    fn new() -> Quote {
        let quotes: Vec<_> = include_str!("../quotes.txt").lines().collect();
        let total = quotes.len();

        let choice = rand::thread_rng().gen_range(0..total);

        Quote {
            text: quotes[choice].to_owned(),
            position: choice + 1, // Readers will likely prefer 1-indexing
            total,
        }
    }
}

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            text,
            position,
            total,
        } = self;
        
        let colored_quote = format!("\"{text}\"").bright_green().bold(); //(TODO: Add customizable colors)
        let output_string = format!("  {colored_quote}\nQuote {position} of {total}");

        write!(f, "{}", output_string)
    }
}
