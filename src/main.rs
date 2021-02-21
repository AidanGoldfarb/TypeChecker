use std::fs;
use std::env;
pub mod parser;
pub mod tokenizer;
pub mod types;
pub mod typechecker;
fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	println!("File: {:?}", filename);

	let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut tokens = tokenizer::tokenize(&contents.trim());
	let ast = parser::parse(&mut tokens);
	let tc = typechecker::typecheck(ast);
	println!("RES: {:?}", tc);
}