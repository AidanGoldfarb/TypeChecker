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
                it.next();
                assert_eq!(it.next().unwrap(), 'r');
                assert_eq!(it.next().unwrap(), 'u');
                assert_eq!(it.next().unwrap(), 'e');
                assert_eq!(it.next().unwrap(), 'C');

                result.push(types::Token::TrueC);
            }
            'f' => {
                //false/fdc
                it.next(); //a
                let tmp = it.next().unwrap(); //a
                                              //println!("its not a its {}", tmp);
                if tmp == 'a' {
                    result.push(types::Token::FalseC);
                    assert_eq!(it.next().unwrap(), 'l');
                    assert_eq!(it.next().unwrap(), 's');
                    assert_eq!(it.next().unwrap(), 'e');
                    assert_eq!(it.next().unwrap(), 'C');
                } else if tmp == 'd' {
                    result.push(types::Token::FdC);
                    it.next(); //C
                               //it.next(); //
                } else if tmp == 'u' {
                	//funT
                	assert_eq!(it.next().unwrap(), 'n');
                    assert_eq!(it.next().unwrap(), 'T');
                    assert_eq!(it.next().unwrap(), '(');
                    let t1;
                    let t2;
                    match it.next().unwrap() {
                    	'n' => {
                    		t1 = types::Mytype::NumT;
                    		assert_eq!(it.next().unwrap(), 'u');
                    		assert_eq!(it.next().unwrap(), 'm');
                    		assert_eq!(it.next().unwrap(), 'T');
                    	}
                    	'b' => {
                    		t1 = types::Mytype::BoolT;
                    		assert_eq!(it.next().unwrap(), 'o');
                    		assert_eq!(it.next().unwrap(), 'o');
                    		assert_eq!(it.next().unwrap(), 'l');
                    		assert_eq!(it.next().unwrap(), 'T');
                    	}
                    	_ => {t1 = types::Mytype::BoolT;},
                    }
                    it.next();
                    match it.next().unwrap() {
                    	'n' => {
                    		t2 = types::Mytype::NumT;
                    		assert_eq!(it.next().unwrap(), 'u');
                    		assert_eq!(it.next().unwrap(), 'm');
                    		assert_eq!(it.next().unwrap(), 'T');
                    	}
                    	'b' => {
                    		t2 = types::Mytype::BoolT;
                    		assert_eq!(it.next().unwrap(), 'o');
                    		assert_eq!(it.next().unwrap(), 'o');
                    		assert_eq!(it.next().unwrap(), 'l');
                    		assert_eq!(it.next().unwrap(), 'T');
                    	}
                    	_ => {t2 = types::Mytype::BoolT;},
                    }
                    result.push(types::Token::FunT(t1.clone(),t2.clone()));

                }

                 else {
                    panic!("bad")
                }
            }
            'n' => {
                //numC
                let mut val = String::new();
                let mut flag = 0;
                while it.peek() != Some(&')')  && flag == 0{
                    if it.peek().unwrap().is_digit(10) {
                        val.push(*it.peek().unwrap());
                    }
                    match it.peek().unwrap(){
                    	'T' => {
                    		println!("HERERER:");             
                    		flag = 1;
	                    	result.push(types::Token::NumT);
	                    	()
	                    }
	                    _ => {
	                    	//println!("ITTT:  {:?}", it.peek());
	                    	()
	                    },
                    }
                    it.next();
                }
                it.next();
                if flag == 0 {
	                result.push(types::Token::NumC(val.parse::<i32>().unwrap()));
	            }
            }
            'p' => {
                //plusC
                result.push(types::Token::PlusC);
                it.next(); //
                assert_eq!(it.next().unwrap(), 'l');
                assert_eq!(it.next().unwrap(), 'u');
                assert_eq!(it.next().unwrap(), 's');
                assert_eq!(it.next().unwrap(), 'C');
            }
            'm' => {
                //multC
                result.push(types::Token::MultC);
                it.next(); //
                assert_eq!(it.next().unwrap(), 'u');
                assert_eq!(it.next().unwrap(), 'l');
                assert_eq!(it.next().unwrap(), 't');
                assert_eq!(it.next().unwrap(), 'C');
            }
            'e' => {
                //eqC
                result.push(types::Token::EqC);
                it.next(); //q
                assert_eq!(it.next().unwrap(), 'q');
                assert_eq!(it.next().unwrap(), 'C');
                // it.next(); //C
                // it.next(); //
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
                assert_eq!(it.next().unwrap(), 'p');
                assert_eq!(it.next().unwrap(), 'p');
                assert_eq!(it.next().unwrap(), 'C');
                // it.next(); //p
                // it.next(); //c
                // it.next(); //
            }
            // 'r' => {
            //     //recC
            //     result.push(types::Token::RecC);
            //     it.next(); //e
            //     assert_eq!(it.next().unwrap(), 'e');
            //     assert_eq!(it.next().unwrap(), 'c');
            //     assert_eq!(it.next().unwrap(), 'C');
            //     // it.next(); //c
            //     // it.next(); //C
            //     // it.next(); //
            // }
            '\"' => {
                //string
                let mut string = String::new();
                it.next();
                while it.peek() != Some(&'\"') {
                    string.push(*it.peek().unwrap());
                    it.next();
                }
                it.next();
                result.push(types::Token::Str(string));
            }
            'b' => {
            	//boolT
            	it.next();
	        	assert_eq!(it.next().unwrap(), 'o');
	        	assert_eq!(it.next().unwrap(), 'o');
	        	assert_eq!(it.next().unwrap(), 'l');
	        	assert_eq!(it.next().unwrap(), 'T');
				result.push(types::Token::BoolT);
            }
            ' ' => {
                //handle whitespace
                it.next();
            }
            _ => {
            	//println!("result {:?}", result);
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
    fn testType() {
        let mut vec = Vec::new();
        vec.push(types::Token::NumT);
        let res = tokenize(&String::from("numT"));
        assert_eq!(vec, res);
    }


    #[test]
    fn test0() {
        let mut vec = Vec::new();
        vec.push(types::Token::IdC);
        vec.push(types::Token::LeftParen);
        vec.push(types::Token::Str("hello".to_string()));
        vec.push(types::Token::RightParen);
        let res = tokenize(&String::from("idC(\"hello\")"));
        assert_eq!(vec, res);
    }



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
