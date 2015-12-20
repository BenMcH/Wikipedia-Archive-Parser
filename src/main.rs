use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;

fn main() {
	let mut input_path: String = String::new();
	let mut output_path = String::new();
	let mut input = std::io::stdin();
	let mut vec = vec![String::from("<page>"),String::from("</page>")];
	println!("Enter the path of the input file: ");
	input.read_line(&mut input_path).unwrap();
	let input_path = input_path.trim();
    let file = File::open(&input_path).expect("Failed to open file");
	println!("Enter the path of the output file: ");
	input.read_line(&mut output_path).unwrap();
	let output_path = output_path.trim();	
    let n_file = File::create(&output_path).expect("Failed to create file");
    get_options(&mut input, &mut vec);
    let reader = BufReader::new(file);
    let mut writer = BufWriter::new(n_file);
    let mut in_text = false;
    let mut line_iter = reader.lines();
	while let Some(l1) = line_iter.next(){
		let line = l1.unwrap();
		if should_write(&line, &mut in_text, &vec) {
			writer.write(line.as_bytes()).unwrap();
			writer.write(b"\n").unwrap(); 
		}
	}
}

fn should_write(x: &str, in_text: &mut bool, list : &Vec<String>) -> bool {
	if x.contains("<text") {
		*in_text = true;
	} else if x.contains("</text>") {
		*in_text = false;
	}
	list.iter().any(|tag| x.contains(tag))
}

fn get_options(input: &mut std::io::Stdin, vec : &mut Vec<String>) -> () {
	let mut option = String::new();
	println!("Capture title? y/n?");
	input.read_line(&mut option).unwrap();
	if option.trim().eq("y") {
		println!("here");
		vec.push(String::from("<title>"));
	}
	option.clear();
	println!("Capture redirect? y/n?");
	input.read_line(&mut option).unwrap();
	if option.trim().eq("y") {
		vec.push(String::from("<redirect"));
	}
	option.clear();
	println!("Capture timestamp? y/n?");
	input.read_line(&mut option).unwrap();
	if option.trim().eq("y") {
		vec.push(String::from("<timestamp>"));
	}
	option.clear();
	println!("Capture username? y/n?");
	input.read_line(&mut option).unwrap();
	if option.trim().eq("y") {
		vec.push(String::from("<contributor>"));
		vec.push(String::from("</contributor>"));
		vec.push(String::from("<username>"));
	}
	option.clear();
	println!("Capture text? y/n?");
	input.read_line(&mut option).unwrap();
	if option.trim().eq("y") {
		vec.push(String::from("<text"));
		vec.push(String::from("</text>"));
	}
}