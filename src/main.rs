use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;

fn main() {
    let file = File::open("path to wikipedia xml dumps").expect("Failed to open file");
    let n_file = File::create("export path").expect("Failed to create file");
    let reader = BufReader::new(file);
    let mut writer = BufWriter::new(n_file);
    let mut in_text = false;
    let mut line_iter = reader.lines();
	while let Some(l1) = line_iter.next(){
		let line = l1.unwrap();
		if should_write(&line, &mut in_text) {
			writer.write(line.as_bytes());
			writer.write(b"\n");
		}
	}
}

fn should_write(x: &str, in_text: &mut bool) -> bool {
        if x.contains("<text") {
            *in_text = true;
 			true
 		} else if x.contains("</text>") {
 		    *in_text = false;
 			true
 		}else{
	 		*in_text || x.contains("<page>") || x.contains("<title>") || x.contains("</page>")
 		}
    }