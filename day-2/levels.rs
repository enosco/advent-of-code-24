use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let file = File::open(&args[1])?;
    let input = BufReader::new(file);
    
    let mut safe_count = 0;

    let mut levels: Vec<i32>;
    
    for line in input.lines() {
	let line = line.unwrap();
	levels = line
	    .split(" ")
	    .map(|x| x.parse().unwrap())
	    .collect();

	let mut safe = true;
	let mut prev_diff = levels[1] - levels[0]; 
	for i in 0..levels.len()-1 {

	    let diff = levels[i+1] - levels[i];
	    
	    if (diff ^ prev_diff) < 0 { // XOR is neg if signs differ
		safe = false;
		break;
	    }
	    
	    let rate = diff.abs();
	    if rate < 1 || rate > 3 { // rate of change is out of bounds
		safe = false;
		break;
	    }
	    prev_diff = diff;
	}
	
	if safe {
	    safe_count += 1;
	}	
    }

    println!("{}", safe_count);

    Ok(())
}

