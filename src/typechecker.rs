use crate::types;
use std::collections::HashMap;
use crate::types::Ast::*;

pub fn typecheck(ex: types::Ast) -> types::Mytype {
	let mut env = HashMap::new();
	typecheck_helper(ex,&mut env)
}

pub fn typecheck_helper(input_ast: types::Ast, env: &mut HashMap<String,types::Mytype>) -> types::Mytype {
	match input_ast{
		NumCNode {val: _} => {
			types::Mytype::NumT
		}
		PlusCNode {left,right} => {
			let t1 = typecheck_helper(*left, env);
			let t2 = typecheck_helper(*right, env);
			assert_eq!(t1,types::Mytype::NumT);
			assert_eq!(t1,t2);
			types::Mytype::NumT
		}
		MultCNode {left,right} => {
			let t1 = typecheck_helper(*left, env);
			let t2 = typecheck_helper(*right,env);
			assert_eq!(t1,types::Mytype::NumT);
			assert_eq!(t1,t2);
			types::Mytype::NumT
		}
		TrueCNode => {
			types::Mytype::BoolT
		}
		FalseCNode => {
			types::Mytype::BoolT
		}
		EqCNode {left,right} => {
			let t1 = typecheck_helper(*left, env);
			let t2 = typecheck_helper(*right,env);
			assert_eq!(t1,t2);
			t1
		}
		IfCNode {cond,first,second} => {
			let t1 = typecheck_helper(*cond,env);
			let t2 = typecheck_helper(*first,env);
			let t3 = typecheck_helper(*second,env);
			assert_eq!(t1,types::Mytype::BoolT);
			assert_eq!(t2,t3);
			t2
		}
		IdCNode {strval}=> {
			(*env.get(&strval).unwrap()).clone()
		}
		AppCNode {left,right} => {
			let arg = typecheck_helper(*left,env);
			let fun = typecheck_helper(*right,env);
			let fp1;
			let fp2;
			match fun.clone(){
				types::Mytype::FunT{arg1,arg2} => {
					fp1 = arg1;
					fp2 = arg2;
				}
				_ => panic!("not a function")
			}
			assert_eq!(arg,*fp1);
			assert_eq!(arg,*fp2);
			fun
		}
		FdCNode {strval, t1, t2, ex} => { //fdC(a, at, rt, b)
			env.insert((*strval).to_string(), *t1.clone());
			let bt = typecheck_helper(*ex,env);
			let fp1;
			//let mut fp2 = bt.clone();
			match bt.clone(){
				types::Mytype::FunT{arg1,arg2: _} => {
					fp1 = *arg1;
					//fp2 = *arg2;
				}
				_ => {
					fp1 = types::Mytype::BoolT;
					//fp2 = types::Mytype::BoolT;
					()
				}
			}
			assert_eq!(fp1,*t2);
			env.remove(&(*strval).to_string());
			types::Mytype::FunT{arg1: t1, arg2: t2}
		}
		_ => {
			panic!("rec?");
		}
	}
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer;
    use crate::parser;

    #[test]
    fn test_true_c() {
        let mut res = tokenizer::tokenize(&String::from("trueC"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::BoolT);
    }

    #[test]
    #[should_panic]
    fn test_true_c_fail() {
        let mut res = tokenizer::tokenize(&String::from("trMeC"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::BoolT);
    }

    #[test]
    fn test_false_c() {
        let mut res = tokenizer::tokenize(&String::from("falseC"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::BoolT);
    }

    #[test]
    #[should_panic]
    fn test_false_c_fail() {
        let mut res = tokenizer::tokenize(&String::from("falZeC"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::BoolT);
    }

    #[test]
    fn test_num_c() {
        let mut res = tokenizer::tokenize(&String::from("numC(2)"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::NumT);
    }

    #[test]
    #[should_panic]
    fn test_num_c_fail() {
        let mut res = tokenizer::tokenize(&String::from("numC(falseC)"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::NumT);
    }

    #[test]
    fn test_plus_c() {
        let mut res = tokenizer::tokenize(&String::from("plusC(numC(2),numC(3))"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::NumT);
    }

    #[test]
    fn test_mult_c() {
        let mut res = tokenizer::tokenize(&String::from("multC(numC(2),numC(3))"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::NumT);
    }

    #[test]
    fn test_eq_c1() {
        let mut res = tokenizer::tokenize(&String::from("eqC(numC(2),numC(3))"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::NumT);
    }

    #[test]
    fn test_eq_c2() {
        let mut res = tokenizer::tokenize(&String::from("eqC(trueC,falseC)"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::BoolT);
    }

    #[test]
    fn test_if_c1() {
        let mut res = tokenizer::tokenize(&String::from("ifC(trueC,numC(2),numC(3))"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::NumT);
    }

    #[test]
    fn test_if_c2() {
        let mut res = tokenizer::tokenize(&String::from("ifC(trueC,trueC,trueC)"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::BoolT);
    }

    #[test]
    #[should_panic]
    fn test_id_c_fail() {
        let mut res = tokenizer::tokenize(&String::from("idC(\"hello!\")"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::BoolT);
    }

    #[test]
    #[should_panic]
    fn test_app_c() {
        let mut res = tokenizer::tokenize(&String::from("appC(trueC,trueC)"));
        let tree = parser::parse(&mut res);
   		let t = typecheck(tree);
        assert_eq!(t,types::Mytype::BoolT);
    }

    // #[test]
    // fn test_fd_c() {
    //     let mut res = tokenizer::tokenize(&String::from("fdC(\"name\",boolT,boolT,trueC)"));
    //     let tree = parser::parse(&mut res);
   	// 	let t = typecheck(tree);
    //     assert_eq!(t,types::Mytype::BoolT);
    // }
 }	