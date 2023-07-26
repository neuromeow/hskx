use std::error::Error;

use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?;
    println!("{:?}", headers);
    for record in reader.records() {
        let unwrapped_record = record?;
        println!("{:?}", unwrapped_record);
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    read_from_file("./src/data/wordlist.csv")?;
    Ok(())
}
