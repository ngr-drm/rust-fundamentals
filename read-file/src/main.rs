use csv::Reader;
use std::{error::Error, process};

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("example.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
