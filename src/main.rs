use std::error::Error;
use std::io::{prelude::*, BufReader, SeekFrom};
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use rand::Rng;

fn main() {
    let quote = Quote::from("./quotes.txt").expect("There was a problem fetching the quote");
    println!("{}", quote);
}

struct Quote {
    text: String,
    position: usize,
    total: usize
}

impl Quote {
    fn from(filename: &str) -> Result<Quote, Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut buffer = BufReader::new(&file);

        // count total lines in file
        let total = (&mut buffer).lines().count();

        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(1..=total) as usize;

        // get the nth line in file
        let mut text = String::new();
        let mut position = 0;
        buffer.seek(SeekFrom::Start(0))?;
        for (n, line) in buffer.lines().enumerate() {
            if n + 1 == choice {
                text = line?;
                position = n + 1;
                break;
            }
        }
        Ok(Quote {text: text, position: position, total: total})
    }
}

impl Display for Quote {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "  \"{}\"\nQuote {} of {}", self.text, self.position, self.total)
    }
}