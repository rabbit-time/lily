use std::error::Error;
use std::{include_bytes};
use std::io::{prelude::*, Cursor, SeekFrom};
use std::fmt::{self, Display, Formatter};
use rand::Rng;

fn main() {
    let quote = Quote::from().expect("There was a problem fetching the quote");
    println!("{}", quote);
}

struct Quote {
    text: String,
    position: usize,
    total: usize
}

impl Quote {
    fn from() -> Result<Quote, Box<dyn Error>> {
        let file = include_bytes!("../quotes.txt");
        let mut cursor = Cursor::new(file);

        // count total lines in file
        let total = (&mut cursor).lines().count();

        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(1..=total) as usize;

        // get the nth line in file
        let mut text = String::new();
        let mut position = 0;
        cursor.seek(SeekFrom::Start(0))?;
        for (n, line) in file.lines().enumerate() {
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