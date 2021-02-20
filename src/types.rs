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

#[derive(Debug, PartialEq, Clone, Copy)]
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
    TrueC {
        val: Box<bool>,
    },
    FalseC {
        val: Box<bool>,
    },
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
        strval: Box<String>,
    },
    AppC {
        left: Box<Ast>,
        right: Box<Ast>,
    },
    FdC {
        strval: Box<String>,
        t1: Box<Mytype>,
        t2: Box<Mytype>,
        ex: Box<Ast>,
    },
    RecC {
        str1v: Box<String>,
        str2v: Box<String>,
        t1: Box<Mytype>,
        t2: Box<Mytype>,
        ex1: Box<Ast>,
        ex2: Box<Ast>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Mytype {
    NumT,
    BoolT,
    FunT { t1: Box<Mytype>, t2: Box<Mytype> },
}
