use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::BinaryHeap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let file = File::open(&args[1])?;
    let input = BufReader::new(file);

    let mut l_heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut r_heap: BinaryHeap<i32> = BinaryHeap::new();

    let mut cols: Vec<&str>;
    for line in input.lines() {

	let line = line.unwrap();
	cols = line.split("   ").collect();

	l_heap.push(cols[0].parse()?);
	r_heap.push(cols[1].parse()?);
	
    }

    let mut diff_total: i32 = 0;
    while !l_heap.is_empty() {

	let left = l_heap.pop().unwrap();
	let right = r_heap.pop().unwrap();
	
	diff_total += i32::abs(left - right);
	
    }
    
    println!("{}", diff_total);
    
    Ok(())
}
