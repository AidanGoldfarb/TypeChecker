use crate::types;
use std::collections::HashMap;

pub fn typecheck(ex: types::Ast) -> types::Mytype {
	let env = HashMap::new();
	typecheck_helper(ex,env)
}

pub fn typecheck_helper(input_ast: types::Ast, env: HashMap<String,types::Mytype>) -> types::Mytype {
	match input_ast
}	