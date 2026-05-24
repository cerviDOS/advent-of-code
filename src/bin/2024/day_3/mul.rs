use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let file = File::open(&args[1])?;
    let input = BufReader::new(file);
      
    for line in input.lines() {

	let line = line.unwrap();
	

	
    }

    
    println!("{}", diff_total);
    
    Ok(())
}
