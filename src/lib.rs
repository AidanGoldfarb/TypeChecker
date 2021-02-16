//use std::iter::Peekable;

pub enum TyExprC{
	NumC(i32),
	PlusC,
	MultC,
	TrueC,
	FalseC,
	EqC,
	IfC,
	IdC,
	AppC,
	FdC,
	RecC
}

pub enum Type{
	NumT,
	BoolT,
	FunT,
}

pub struct Node{
	pub children: Vec<Node>,
	pub val: TyExprC,
}
impl Node{
	pub fn new() -> Node{
		Node{
			children: Vec::new(),
			val: TyExprC::NumC(0),
		}
	}
}

fn lex(input_str: &String) -> Result<Vec<Type>, String>{
	let mut result = Vec::new();
	let mut tolkens = input_str.chars().peekable();
	while let Some(&c) = tolkens.peek(){
		match c{
			'n' => {
				//handle numT
				result.push(Type::NumT);
				tolkens.next();
				tolkens.next();
				tolkens.next();
			}
			'b' => {
				//handle bool
				result.push(Type::BoolT);
				tolkens.next();
				tolkens.next();
				tolkens.next();
				tolkens.next();
			}
			'f' => {
				//handle funT
				result.push(Type::FunT);
				tolkens.next();
				tolkens.next();
				tolkens.next();
			}
			' ' => {
				//handle whitespace
				tolkens.next();
			}
			_ => {
				return Err(format!("wtf is this??: {}", c));
			}
		}
	}
	Ok(result)
}