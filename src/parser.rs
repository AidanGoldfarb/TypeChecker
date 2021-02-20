use crate::types;

pub fn parse(input: &mut Vec<types::Token>) -> bool {
    println!("Im at the start and input: {:?}", input);
    let mut cur_token = input[0];
    while !input.is_empty() {
        match cur_token {
            types::Token::LeftParen => {
                input.remove(0);
                if input.is_empty() {
                    return false;
                }
                cur_token = input[0];
            }
            types::Token::RightParen => {
                input.remove(0);
                if input.is_empty() {
                    return true;
                }
                cur_token = input[0];
            }
            types::Token::Comma => {
                input.remove(0);
                if input.is_empty() {
                    return false;
                }
                cur_token = input[0];
            }
            types::Token::TrueC => {
                input.remove(0);
                return true;
            }
            types::Token::FalseC => {
                input.remove(0);
                return true;
            }
            types::Token::NumC(_) => {
                input.remove(0);
                return true;
            }
            types::Token::PlusC => {
                input.remove(0);
                assert_eq!(input.remove(0), types::Token::LeftParen); //(
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::RightParen); //)
            }
            types::Token::MultC => {
                input.remove(0);
                assert_eq!(input.remove(0), types::Token::LeftParen); //(
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::RightParen); //)
            }
            types::Token::EqC => {
                input.remove(0);
                assert_eq!(input.remove(0), types::Token::LeftParen); //(
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::RightParen); //)
            }
            types::Token::IfC => {
                input.remove(0);
                assert_eq!(input.remove(0), types::Token::LeftParen); //(
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //)
                parse(input);
                assert_eq!(input.remove(0), types::Token::RightParen);
            }
            types::Token::IdC => {
                input.remove(0);
                assert_eq!(input.remove(0), types::Token::LeftParen); //(
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::RightParen); //,
            }
            types::Token::AppC => {
                input.remove(0);
                assert_eq!(input.remove(0), types::Token::LeftParen); //(
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::RightParen); //)
            }
            types::Token::FdC => {
                input.remove(0);
                assert_eq!(input.remove(0), types::Token::LeftParen); //(
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::RightParen); //)
            }
            types::Token::RecC => {
                input.remove(0);
                assert_eq!(input.remove(0), types::Token::LeftParen); //(
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input);
                assert_eq!(input.remove(0), types::Token::Comma); //
                parse(input);
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::RightParen); //)
            } // _ => {
              // 	panic!("Invalid syntax");
              // }
        }
        //cur_token = input.remove(0);
        println!("cur_token after match: {:?}", cur_token);
        return true;
    }
    println!("Returing");
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer;
    // #[test]
    //    fn test1() {
    //    	let res = tokenizer::tokenize(&String::from("(((ifC(trueC,ifC(trueC,falseC)))))"));
    //        assert!(parse(res));
    //    }

    #[test]
    fn test_plus_c() {
        let mut res = tokenizer::tokenize(&String::from("plusC(numC(2),numC(3))"));
        assert!(parse(&mut res));
    }
    #[test]
    fn test_mult_c() {
        let mut res = tokenizer::tokenize(&String::from("multC(numC(2),numC(3))"));
        assert!(parse(&mut res));
    }
    #[test]
    fn test_true_c() {
        let mut res = tokenizer::tokenize(&String::from("trueC"));
        assert!(parse(&mut res));
    }
    #[test]
    fn test_false_c() {
        let mut res = tokenizer::tokenize(&String::from("falseC"));
        assert!(parse(&mut res));
    }
    #[test]
    fn test_eq_c() {
        let mut res = tokenizer::tokenize(&String::from("eqC(numC(2),numC(2))"));
        assert!(parse(&mut res));
    }
    #[test]
    fn test_if_c() {
        let mut res = tokenizer::tokenize(&String::from("ifC(trueC,numC(3),num(4))"));
        assert!(parse(&mut res));
    }
    #[test]
    fn test_id_c() {
        let mut res = tokenizer::tokenize(&String::from("idC(trueC)"));
        assert!(parse(&mut res));
    }
    #[test]
    fn test_app_c() {
        let mut res = tokenizer::tokenize(&String::from("appC(trueC,numC(2))"));
        assert!(parse(&mut res));
    }
    #[test]
    fn test_fd_c() {
        let mut res = tokenizer::tokenize(&String::from("fdC(trueC,trueC,trueC,trueC)"));
        assert!(parse(&mut res));
    }
    #[test]
    fn test_rec_c() {
        let mut res =
            tokenizer::tokenize(&String::from("recC(trueC,trueC,trueC,trueC,trueC,trueC)"));
        assert!(parse(&mut res));
    }

    #[test]
    fn test_combo_1() {
        let mut res = tokenizer::tokenize(&String::from("eqC(eqC(numC(1),numC(2)),numC(3))"));
        assert!(parse(&mut res));
    }

    #[test]
    fn test_combo_2() {
        let mut res = tokenizer::tokenize(&String::from(
            "eqC(eqC(numC(1),numC(2)),eqC(numC(3),numC(4)))",
        ));
        assert!(parse(&mut res));
    }

    #[test]
    fn test_combo_3() {
        let mut res = tokenizer::tokenize(&String::from(
            "eqC(eqC(numC(1),numC(2)),ifC(trueC,num(3),num(4)))",
        ));
        assert!(parse(&mut res));
    }
    // #[test]
    // fn test3() {
    //     let mut res = tokenizer::tokenize(&String::from("pluC(numC(2),numC(3))"));
    //     assert_eq!(parse(&mut res),false);
    // }
}
