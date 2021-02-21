use crate::types;

pub fn parse(input: &mut Vec<types::Token>) -> types::Ast {
    let cur_token = input[0].clone();
    // if !input.is_empty() {
    //     panic!("empty");
    // }
    input.remove(0);
    match cur_token {
        types::Token::TrueC => types::Ast::TrueCNode,
        types::Token::FalseC => types::Ast::FalseCNode,
        types::Token::NumC(_) => types::Ast::NumCNode { val: Box::new(0) },
        types::Token::PlusC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t2 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::RightParen); //)
            types::Ast::PlusCNode {
                left: Box::new(t1),
                right: Box::new(t2),
            }
        }
        types::Token::MultC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t2 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::RightParen); //)
            types::Ast::MultCNode {
                left: Box::new(t1),
                right: Box::new(t2),
            }
        }
        types::Token::EqC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t2 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::RightParen); //)
            types::Ast::EqCNode {
                left: Box::new(t1),
                right: Box::new(t2),
            }
        }
        types::Token::IfC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t2 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //)
            let t3 = parse(input);
            assert_eq!(input.remove(0), types::Token::RightParen);
            types::Ast::IfCNode {
                cond: Box::new(t1),
                first: Box::new(t2),
                second: Box::new(t3),
            }
        }
        types::Token::IdC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            //let t1 = parse(input); //Str
            //let strr = input[0];
            let strr = input.remove(0);//assert_eq!(input.remove(0), types::Token::Str(_));
            let strrr;
            match strr{
            	types::Token::Str(c) => {strrr = c},
            	_ => {strrr = "ligma".to_string()},
            }
            assert_eq!(input.remove(0), types::Token::RightParen); //,
            types::Ast::IdCNode {
                strval: String::from(strrr),
            }
        }
        types::Token::AppC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t2 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::RightParen); //)
            types::Ast::AppCNode {
                left: Box::new(t1),
                right: Box::new(t2),
            }
        }
        types::Token::FdC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let strr = input.remove(0);
            let strrr;
            match strr{
            	types::Token::Str(c) => {strrr = c},
            	_ => {strrr = "ligma".to_string()},
            }
            //let t1 = parse(input); //expr
            //assert_eq!(strr, types::Token::Str(_));
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let type_token1 = input.remove(0);//let t2 = parse(input); //expr
            let  tt1;
            match type_token1{
            	types::Token::NumT => {
            		tt1 = types::Mytype::NumT;
            	}
            	types::Token::BoolT => {
            		tt1 = types::Mytype::BoolT;
            	}
            	types::Token::FunT(arg1,arg2) => {
            		tt1 = types::Mytype::FunT{arg1: Box::new(arg1),arg2: Box::new(arg2)};
            	}
            	_ => {
            		tt1 = types::Mytype::NumT;
            	}
            }
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let type_token2 = input.remove(0);//let t3 = parse(input); //expr
            let  tt2;
            match type_token2{
            	types::Token::NumT => {
            		tt2 = types::Mytype::NumT;
            	}
            	types::Token::BoolT => {
            		tt2 = types::Mytype::BoolT;
            	}
            	types::Token::FunT(arg1,arg2) => {
            		tt2 = types::Mytype::FunT{arg1: Box::new(arg1),arg2: Box::new(arg2)};
            	}
            	_ => {
            		tt2 = types::Mytype::NumT;
            	}
            }
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t4 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::RightParen); //)
            types::Ast::FdCNode {
                strval: String::from(strrr),
                t1: Box::new(tt1),
                t2: Box::new(tt2),
                ex: Box::new(t4),
            }
        }
        // types::Token::RecC => {
        //     assert_eq!(input.remove(0), types::Token::LeftParen); //(
        //     let t1 = parse(input); //expr
        //     assert_eq!(input.remove(0), types::Token::Comma); //,
        //     let t2 = parse(input); //expr
        //     assert_eq!(input.remove(0), types::Token::Comma); //,
        //     let t3 = parse(input);
        //     assert_eq!(input.remove(0), types::Token::Comma); //
        //     let t4 = parse(input);
        //     assert_eq!(input.remove(0), types::Token::Comma); //,
        //     let t5 = parse(input); //expr
        //     assert_eq!(input.remove(0), types::Token::Comma); //,
        //     let t6 = parse(input); //expr
        //     assert_eq!(input.remove(0), types::Token::RightParen); //)
        //     types::Ast::RecCNode {
        //         str1v: Box::new(t1),
        //         str2v: Box::new(t2),
        //         t1: Box::new(t3),
        //         t2: Box::new(t4),
        //         ex1: Box::new(t5),
        //         ex2: Box::new(t6),
        //     }
        // }

        _ => panic!("Wrong syntax"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer;

    #[test]
    fn test1() {
        let mut res = tokenizer::tokenize(&String::from("trueC"));
        let tree = parse(&mut res);
        println!("\n\n\nTree: {:#?}\n\n\n", tree);
        assert!(true);
    }
    // #[test]
    //    fn test1() {
    //    	let res = tokenizer::tokenize(&String::from("(((ifC(trueC,ifC(trueC,falseC)))))"));
    //        assert!(parse(res));
    //    }

    // #[test]
    // fn test_plus_c() {
    //     let mut res = tokenizer::tokenize(&String::from("plusC(numC(2),numC(3))"));
    //     assert!(parse(&mut res));
    // }
    // #[test]
    // fn test_mult_c() {
    //     let mut res = tokenizer::tokenize(&String::from("multC(numC(2),numC(3))"));
    //     assert!(parse(&mut res));
    // }
    // #[test]
    // fn test_true_c() {
    //     let mut res = tokenizer::tokenize(&String::from("trueC"));
    //     assert!(parse(&mut res));
    // }
    // #[test]
    // fn test_false_c() {
    //     let mut res = tokenizer::tokenize(&String::from("falseC"));
    //     assert!(parse(&mut res));
    // }
    // #[test]
    // fn test_eq_c() {
    //     let mut res = tokenizer::tokenize(&String::from("eqC(numC(2),numC(2))"));
    //     assert!(parse(&mut res));
    // }
    // #[test]
    // fn test_if_c() {
    //     let mut res = tokenizer::tokenize(&String::from("ifC(trueC,numC(3),num(4))"));
    //     assert!(parse(&mut res));
    // }
    // #[test]
    // fn test_id_c() {
    //     let mut res = tokenizer::tokenize(&String::from("idC(trueC)"));
    //     assert!(parse(&mut res));
    // }
    // #[test]
    // fn test_app_c() {
    //     let mut res = tokenizer::tokenize(&String::from("appC(trueC,numC(2))"));
    //     assert!(parse(&mut res));
    // }
    // #[test]
    // fn test_fd_c() {
    //     let mut res = tokenizer::tokenize(&String::from("fdC(trueC,trueC,trueC,trueC)"));
    //     assert!(parse(&mut res));
    // }
    // #[test]
    // fn test_rec_c() {
    //     let mut res =
    //         tokenizer::tokenize(&String::from("recC(trueC,trueC,trueC,trueC,trueC,trueC)"));
    //     assert!(parse(&mut res));
    // }

    // #[test]
    // fn test_combo_1() {
    //     let mut res = tokenizer::tokenize(&String::from("eqC(eqC(numC(1),numC(2)),numC(3))"));
    //     assert!(parse(&mut res));
    // }

    // #[test]
    // fn test_combo_2() {
    //     let mut res = tokenizer::tokenize(&String::from(
    //         "eqC(eqC(numC(1),numC(2)),eqC(numC(3),numC(4)))",
    //     ));
    //     assert!(parse(&mut res));
    // }

    // #[test]
    // fn test_combo_3() {
    //     let mut res = tokenizer::tokenize(&String::from(
    //         "eqC(eqC(numC(1),numC(2)),ifC(trueC,num(3),num(4)))",
    //     ));
    //     assert!(parse(&mut res));
    // }
    // // #[test]
    // // fn test3() {
    // //     let mut res = tokenizer::tokenize(&String::from("pluC(numC(2),numC(3))"));
    // //     assert_eq!(parse(&mut res),false);
    // // }
}
