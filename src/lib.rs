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

pub fn parse(input: &String) -> Result<Node, String>{
	let tokens = lex(input)?;
    parse_expr(&tokens, 0).and_then(|(n, i)| if i == tokens.len() {
        Ok(n)
    } else {
        Err(format!("Expected end of input, found {:?} at {}", tokens[i], i))
    })
}

fn parse_expr(tokens: &Vec<Type>, pos: usize) -> Result<(Node, usize), String> {
	let (node_summand, next_pos) = parse_summand(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('+')) => {
            // recurse on the expr
            let mut sum = ParseNode::new();
            sum.entry = GrammarItem::Sum;
            sum.children.push(node_summand);
            let (rhs, i) = parse_expr(tokens, next_pos + 1)?;
            sum.children.push(rhs);
            Ok((sum, i))
        }
        _ => {
            // we have just the summand production, nothing more.
            Ok((node_summand, next_pos))
        }
    }
}

fn parse_summand(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node_term, next_pos) = parse_term(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('*')) => {
            // recurse on the summand
            let mut product = ParseNode::new();
            product.entry = GrammarItem::Product;
            product.children.push(node_term);
            let (rhs, i) = parse_summand(tokens, next_pos + 1)?;
            product.children.push(rhs);
            Ok((product, i))
        }
        _ => {
            // we have just the term production, nothing more.
            Ok((node_term, next_pos))
        }
    }
}