use tc::tokenizer;

fn main() {
    let teststr = String::from("(asdfsdaf)");
	let tokens = tokenizer::tokenize(&teststr);  
	println!("{:?}", tokens);
}
