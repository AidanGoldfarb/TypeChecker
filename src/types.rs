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
    //RecC,
    Str(String),
    NumT,
    FunT(Mytype,Mytype),
    BoolT,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Ast {
    NumCNode {
        val: Box<i32>,
    },
    PlusCNode {
        left: Box<Ast>,
        right: Box<Ast>,
    },
    MultCNode {
        left: Box<Ast>,
        right: Box<Ast>,
    },
    TrueCNode,
    FalseCNode,
    EqCNode {
        left: Box<Ast>,
        right: Box<Ast>,
    },
    IfCNode {
        cond: Box<Ast>,
        first: Box<Ast>,
        second: Box<Ast>,
    },
    IdCNode {
        strval: String,
    },
    AppCNode {
        left: Box<Ast>,
        right: Box<Ast>,
    },
    FdCNode {
        strval: String,
        t1: Box<Mytype>, //mytype
        t2: Box<Mytype>, //mytype
        ex: Box<Ast>,
    },
    // RecCNode {
    //     str1v: Box<Ast>,
    //     str2v: Box<Ast>,
    //     t1: Box<Mytype>, //mytype
    //     t2: Box<Mytype>, //mytype
    //     ex1: Box<Ast>,
    //     ex2: Box<Ast>,
    // },
    NoNode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mytype {
    NumT,
    BoolT,
    FunT { arg1: Box<Mytype>, arg2: Box<Mytype> },
}
// fdC("arg1", if(true, numT, idC("arg1"))