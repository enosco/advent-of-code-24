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
	    .trim()
	    .split(" ")
	    .map(|x| x.parse().unwrap())
	    .collect(); // parse and format input line
	
	let result = verify_report(&levels); // get result of initial safety test
	
	if result.is_ok() {
	    safe_count += 1;
	} else {
	    let target_idx = result.err().unwrap();
	    if verify_dampened(&levels, target_idx) {		
		safe_count += 1;
	    }
	}
    }
    
    println!("{}", safe_count);
    
    Ok(())
}

// returns Result::Ok containing level length if report is safe,
// otherwise returns Result::Err containing the index of the last checked 
// level
fn verify_report(levels: &Vec<i32>) -> Result<usize, usize> {
    let mut result = Ok(levels.len());
    let mut prev_diff = levels[1] - levels[0];
    
    for i in 0..levels.len()-1 {
	let diff = levels[i+1] - levels[i];
	
	if !is_pair_safe(diff, prev_diff) {
	    result = Err(i+1);
	    break;
	}
	
	prev_diff = diff;   
    }
    
    return result;
}

fn is_pair_safe(curr_diff: i32, prev_diff: i32) -> bool {    
    let rate = curr_diff.abs();

    let safe;
    if (curr_diff ^ prev_diff) < 0 {
	safe = false;
    } else if rate < 1 || rate > 3 {
	safe = false;
    } else {
	safe = true;
    }
    
    return safe;
}

// returns true if report is safe when dampened
fn verify_dampened(levels: &Vec<i32>, index: usize) -> bool {
    // to dampen, remove each level starting at the target index
    for i in (0..=index).rev() { 
	let mut dampened: Vec<i32> = levels.clone();
	dampened.remove(i);
	
	if verify_report(&dampened).is_ok() {
	    return true;
	}		
    }
    
    return false;
}
