use crate::types;

pub fn parse(input: &mut Vec<types::Token>) -> types::Ast {
    let cur_token = input[0].clone();
    if !input.is_empty() {
        panic!("Wrong syntax");
    }
    input.remove(0);
    match cur_token {
        types::Token::TrueC => return types::Ast::TrueC,
        types::Token::FalseC => return types::Ast::FalseC,
        types::Token::NumC(_) => return types::Ast::NumC { val: Box::new(0) },
        types::Token::PlusC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t2 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::RightParen); //)
            return types::Ast::PlusC {
                left: Box::new(t1),
                right: Box::new(t2),
            };
        }
        types::Token::MultC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t2 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::RightParen); //)
            return types::Ast::MultC {
                left: Box::new(t1),
                right: Box::new(t2),
            };
        }
        types::Token::EqC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t2 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::RightParen); //)
            return types::Ast::EqC {
                left: Box::new(t1),
                right: Box::new(t2),
            };
        }
        types::Token::IfC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t2 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //)
            let t3 = parse(input);
            assert_eq!(input.remove(0), types::Token::RightParen);
            return types::Ast::IfC {
                cond: Box::new(t1),
                first: Box::new(t2),
                second: Box::new(t3),
            };
        }
        types::Token::IdC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::RightParen); //,
            return types::Ast::IdC {
                strval: Box::new(t1),
            };
        }
        types::Token::AppC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t2 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::RightParen); //)
            return types::Ast::AppC {
                left: Box::new(t1),
                right: Box::new(t2),
            };
        }
        types::Token::FdC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t2 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t3 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t4 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::RightParen); //)
            return types::Ast::FdC {
                strval: Box::new(t1),
                t1: Box::new(t2),
                t2: Box::new(t3),
                ex: Box::new(t4),
            };
        }
        types::Token::RecC => {
            assert_eq!(input.remove(0), types::Token::LeftParen); //(
            let t1 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t2 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t3 = parse(input);
            assert_eq!(input.remove(0), types::Token::Comma); //
            let t4 = parse(input);
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t5 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::Comma); //,
            let t6 = parse(input); //expr
            assert_eq!(input.remove(0), types::Token::RightParen); //)
            return types::Ast::RecC {
                str1v: Box::new(t1),
                str2v: Box::new(t2),
                t1: Box::new(t3),
                t2: Box::new(t4),
                ex1: Box::new(t5),
                ex2: Box::new(t6),
            };
        }

        _ => panic!("Wrong syntax"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer;

    #[test]
    fn test1() {
       	let res = tokenizer::tokenize(&String::from("recC(trueC,trueC,trueC,trueC,trueC,trueC)"));
       	println!("Tree: {:?}", res);
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
