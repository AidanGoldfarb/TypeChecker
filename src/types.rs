// pub enum TyExprC{
// 	NumC(i32),
// 	PlusC,
// 	MultC,
// 	TrueC,
// 	FalseC,
// 	EqC,
// 	IfC,
// 	IdC,
// 	AppC,
// 	FdC,
// 	RecC
// }

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftParen,  //
    RightParen, //
    Comma,      //
    TrueC,      //
    FalseC,     //
    NumC(i32),  //
    PlusC,      //
    MultC,      //
    EqC,        //
    IfC,        //
    IdC,        //
    AppC,       //
    FdC,        //
    RecC,
    Str(String),
    NumT,
    FunT,
    BoolT,

}

#[derive(Debug, PartialEq)]
pub enum Ast {
    NumC {
        val: Box<i32>,
    },
    PlusC {
        left: Box<Ast>,
        right: Box<Ast>,
    },
    MultC {
        left: Box<Ast>,
        right: Box<Ast>,
    },
    TrueC,
    FalseC,
    EqC {
        left: Box<Ast>,
        right: Box<Ast>,
    },
    IfC {
        cond: Box<Ast>,
        first: Box<Ast>,
        second: Box<Ast>,
    },
    IdC {
        strval: Box<Ast>,
    },
    AppC {
        left: Box<Ast>,
        right: Box<Ast>,
    },
    FdC {
        strval: Box<Ast>,
        t1: Box<Ast>, //mytype
        t2: Box<Ast>, //mytype
        ex: Box<Ast>,
    },
    RecC {
        str1v: Box<Ast>,
        str2v: Box<Ast>,
        t1: Box<Ast>, //mytype
        t2: Box<Ast>, //mytype
        ex1: Box<Ast>,
        ex2: Box<Ast>,
    },
    No,
}

#[derive(Debug, PartialEq)]
pub enum Mytype {
    NumT,
    BoolT,
    FunT { t1: Box<Mytype>, t2: Box<Mytype> },
}
