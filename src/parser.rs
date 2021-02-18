use crate::tokenizer;
use crate::types;

pub fn parse(input: &mut Vec<types::Token>) -> bool {
    println!("Im at the start and input: {:?}", input);
    let mut cur_token = input[0];
    while input.len() > 0 {
        match cur_token {
            types::Token::LeftParen => {
                input.remove(0);
                if input.len() < 1 {
                    return false;
                }
                cur_token = input[0];
            }
            types::Token::RightParen => {
                input.remove(0);
                if input.len() < 1 {
                    return true;
                }
                cur_token = input[0];
            }
            types::Token::Comma => {
                //println!("Found Comma!");
                input.remove(0);
                if input.len() < 1 {
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
                assert_eq!(input.remove(0), types::Token::Comma); //)
                parse(input);
                assert_eq!(input.remove(0), types::Token::Comma); //)
                parse(input);
                assert_eq!(input.remove(0), types::Token::RightParen);
            }
            types::Token::RecC => {
                input.remove(0);
                assert_eq!(input.remove(0), types::Token::LeftParen); //(
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //)
                parse(input);
                assert_eq!(input.remove(0), types::Token::Comma); //)
                parse(input);
                assert_eq!(input.remove(0), types::Token::Comma); //,
                parse(input); //expr
                assert_eq!(input.remove(0), types::Token::Comma); //)
                parse(input);
                assert_eq!(input.remove(0), types::Token::Comma); //)
                parse(input);
                assert_eq!(input.remove(0), types::Token::RightParen);
            }

            _ => {
            	panic!("Very bad");
            }
        }
        //cur_token = input.remove(0);
        println!("cur_token after match: {:?}", cur_token);
    }
    println!("Returing");
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    //    fn test1() {
    //    	let res = tokenizer::tokenize(&String::from("(((ifC(trueC,ifC(trueC,falseC)))))"));
    //        assert!(parse(res));
    //    }

    #[test]
    fn test2() {
        let mut res = tokenizer::tokenize(&String::from("plusC(numC(2),numC(3))"));
        assert!(parse(&mut res));
    }

    #[test]
    fn test3() {
        let mut res = tokenizer::tokenize(&String::from("plusC(numC(2),numC(3))"));
        assert!(parse(&mut res));
    }
}
