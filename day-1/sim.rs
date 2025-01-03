use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let file = File::open(&args[1])?;
    let input = BufReader::new(file);

    let mut l_occur: HashMap<i32,i32> = HashMap::new();
    let mut r_occur: HashMap<i32,i32> = HashMap::new();

    let mut cols: Vec<&str>;
    for line in input.lines() {

	let line = line.unwrap();
	cols = line.split("   ").collect();

	*l_occur.entry(cols[0].parse().unwrap()).or_insert(0) += 1;
	*r_occur.entry(cols[1].parse().unwrap()).or_insert(0) += 1;	
	
    }

    let mut similarity = 0;
    for (key,value) in l_occur.into_iter() {
	let occur = r_occur.get(&key);

	if occur != None {
	    similarity += key * value * occur.unwrap();
	}	   
    }

    println!("SIM_SCORE: {}", similarity);
    
    Ok(())
}
