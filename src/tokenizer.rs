use crate::types;
//use tc::types::Token;
pub fn tokenize(input: &str) -> Vec<types::Token> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
    	//it.next();
        match c {
            '(' => {
                //(
                result.push(types::Token::LeftParen);
                it.next();
            }
            ')' => {
                //)
                result.push(types::Token::RightParen);
                it.next();
            }
            ',' => {
                //,
                result.push(types::Token::Comma);
                it.next();
            }
            't' => {
                //true
                result.push(types::Token::TrueC);
                it.next();
                it.next();
                it.next();
                it.next();
                it.next();
            }
            'f' => {
                //false/fdc
                it.next(); //a
                let tmp = it.next().unwrap(); //a
                                              //println!("its not a its {}", tmp);
                if tmp == 'a' {
                    result.push(types::Token::FalseC);
                    it.next(); //l
                    it.next(); //s
                    it.next(); //e
                    it.next(); //
                } else {
                    result.push(types::Token::FdC);
                    it.next(); //C
                    //it.next(); //
                }
            }
             'n' => { //numC
                let mut val = String::new();
                while it.peek() != Some(&')'){
                	if it.peek().unwrap().is_digit(10){
                		val.push(*it.peek().unwrap());
                	}
                	it.next();
                }
                it.next();
                result.push(types::Token::NumC(val.parse::<i32>().unwrap()));
            }
            // 'n' => {
            //     //numC(val)
            //     let mut val;
            //     it.next(); //u
            //     it.next(); //m
            //     it.next(); //C
            //     it.next(); //(
            //     it.next(); //idk man
            //     let mut n = it.next(); //#
            //     println!("current n: {:?}", n);
            //     val = String::new();
            //     while n != Some(')') {
            //         val.push(n.unwrap());
            //         n = it.next();
            //     }
            //     println!("FOUND AN INT: {:?}", val);
            //     result.push(types::Token::NumC(val.parse::<i32>().unwrap()));
            // }
            'p' => {
                //plusC
                result.push(types::Token::PlusC);
                it.next(); //l
                it.next(); //u
                it.next(); //s
                it.next(); //C
                it.next(); //
            }
            'm' => {
                //multC
                result.push(types::Token::MultC);
                it.next(); //u
                it.next(); //l
                it.next(); //t
                it.next(); //C
                it.next(); //
            }
            'e' => {
                //eqC
                result.push(types::Token::EqC);
                it.next(); //q
                it.next(); //C
                it.next(); //
            }
            'i' => {
                //ifC/idC
                it.next();
                let tmp = it.next().unwrap();
                if tmp == 'f' {
                    result.push(types::Token::IfC);
                } else {
                    result.push(types::Token::IdC);
                }
                //it.next(); // C
                it.next(); //
            }
            'a' => {
                //appC
                result.push(types::Token::AppC);
                it.next(); //p
                it.next(); //p
                it.next(); //c
                it.next(); //
            }
            'r' => {
                //recC
                result.push(types::Token::RecC);
                it.next(); //e
                it.next(); //c
                it.next(); //C
                it.next(); //
            }
            ' ' => {
                //handle whitespace

                it.next();
            }
            _ => {
                panic!("bad")
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut vec = Vec::new();
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::RightParen);
        let res = tokenize(&String::from("()"));
        assert_eq!(vec, res);
    }

    #[test]
    fn test2() {
        let mut vec = Vec::new();
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::IfC);
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::TrueC);
        vec.push(types::Token::Comma);
        vec.push(types::Token::FalseC);
        vec.push(types::Token::RightParen);
        vec.push(types::Token::RightParen);
        let res = tokenize(&String::from("(ifC(trueC,falseC))"));
        assert_eq!(vec, res);
    }

    #[test]
    fn test3() {
        let mut vec = Vec::new();
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::IfC);
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::TrueC);
        vec.push(types::Token::Comma);
        vec.push(types::Token::Comma);
        vec.push(types::Token::FalseC);
        vec.push(types::Token::RightParen);
        vec.push(types::Token::RightParen);
        let res = tokenize(&String::from("(ifC(trueC,,falseC))"));
        assert_eq!(vec, res);
    }

    #[test]
    fn test4() {
        let mut vec = Vec::new();
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::IfC);
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::TrueC);
        vec.push(types::Token::Comma);
        vec.push(types::Token::IfC);
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::TrueC);
        vec.push(types::Token::Comma);
        vec.push(types::Token::FalseC);
        vec.push(types::Token::RightParen);
        vec.push(types::Token::RightParen);
        vec.push(types::Token::RightParen);
        let res = tokenize(&String::from("(ifC(trueC,ifC(trueC,falseC)))"));
        assert_eq!(vec, res);
    }

    #[test]
    fn test5() {
        let mut vec = Vec::new();
        vec.push(types::Token::NumC(2));
        let res = tokenize(&String::from("numC(2)"));
        assert_eq!(vec, res);
    }

    #[test]
    fn test6() {
        let mut vec = Vec::new();
        vec.push(types::Token::NumC(24));
        let res = tokenize(&String::from("numC(24)"));
        assert_eq!(vec, res);
    }

    #[test]
    fn test7() {
        let mut vec = Vec::new();
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::IfC);
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::TrueC);
        vec.push(types::Token::Comma);
        vec.push(types::Token::IfC);
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::NumC(932));
        vec.push(types::Token::Comma);
        vec.push(types::Token::FalseC);
        vec.push(types::Token::RightParen);
        vec.push(types::Token::RightParen);
        vec.push(types::Token::RightParen);
        let res = tokenize(&String::from("(ifC(trueC,ifC(numC(932),falseC)))"));
        assert_eq!(vec, res);
    }

    #[test]
    fn test_fdc_tokenize() {
        let mut vec = Vec::new();
        vec.push(types::Token::FdC);
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::TrueC);
        vec.push(types::Token::Comma);
        vec.push(types::Token::TrueC);
        vec.push(types::Token::Comma);
        vec.push(types::Token::TrueC);
        vec.push(types::Token::Comma);
        vec.push(types::Token::TrueC);
        vec.push(types::Token::RightParen);
        let res = tokenize(&String::from("fdC(trueC,trueC,trueC,trueC)"));
        assert_eq!(vec, res);
    }

    #[test]
    fn test_ifc_tokenize() {
        let mut vec = Vec::new();
        vec.push(types::Token::IfC);
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::TrueC);
        vec.push(types::Token::Comma);
        vec.push(types::Token::NumC(12345));
        vec.push(types::Token::Comma);
        vec.push(types::Token::NumC(67890));
        vec.push(types::Token::RightParen);
        let res = tokenize(&String::from("ifC(trueC,numC(12345),num(67890))"));
        assert_eq!(vec, res);
    }

}
