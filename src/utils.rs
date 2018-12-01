use std::fs::File;
use std::io::BufReader;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::io::BufRead;

pub fn parse(filename: &str) -> Result<Vec<i32>, io::Error> {
    let inputs = File::open(filename)?;
    let reader = BufReader::new(inputs);
    let mut buffer = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let n = line
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        buffer.push(n);
    }

    Ok(buffer)
}
