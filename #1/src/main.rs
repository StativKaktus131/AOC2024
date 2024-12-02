use std::fs;
use std::collections::HashMap;

fn main() {
	println!("solution to puzzle 1 is: {}", solution_one());
	println!("solution to puzzle 2 is: {}", solution_two());
}

fn solution_one() -> u32 {

	// create variables
	let content = fs::read_to_string("res/input.txt").expect("unable to read file path");
	let mut left_list = Vec::<u32>::new();
	let mut right_list = Vec::<u32>::new();
	let mut result = 0u32;

	// fill left and right lists with the split content from the lines
	for line in content.split("\r\n") {
		let split = line.split("   ").collect::<Vec<&str>>();

		let left = split[0].parse::<u32>().unwrap();
		let right = split[1].parse::<u32>().unwrap();

		left_list.push(left);
		right_list.push(right);
	}

	// sort them (ascending)
	left_list.sort();
	right_list.sort();
	
	for i in 0..left_list.len() {
		// next smallest numbers will always share the same index so you can just add them
		result += (left_list[i] as i32 - right_list[i] as i32).abs() as u32
	}

	result
}

fn solution_two() -> u32 {
	
	let mut similarity_scores = HashMap::new();
	let content = fs::read_to_string("res/input.txt").expect("unable to read file path");
	let mut left_list = Vec::<u32>::new();
	let mut right_list = Vec::<u32>::new();
	let mut result = 0u32;

	// fill left and right lists with the split content from the lines
	for line in content.split("\r\n") {
		let split = line.split("   ").collect::<Vec<&str>>();

		let left = split[0].parse::<u32>().unwrap();
		let right = split[1].parse::<u32>().unwrap();

		left_list.push(left);
		right_list.push(right);
	}

	for left_num in &left_list {

		// get all occurences in right list
		let right_occurences = right_list
			.iter()
			.filter(|x| **x == *left_num)
			.count() as u32;

		// create new entry for num only if it doesn't exist
		// probably solved better using entry API but it's 00:43 A.M. and I'm too lazy to look into it. it works.
		if !similarity_scores.contains_key(&left_num) {
			similarity_scores.insert(left_num, left_num * right_occurences);
		} else {
			*similarity_scores.get_mut(&left_num).unwrap() += left_num * right_occurences;
		}
	}

	// add all the cumulative scores for each number to the result
	for (key, value) in similarity_scores {
		result += value;
	}

	result
}
