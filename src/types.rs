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

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token{
	LeftParen, //
	RightParen, //
	Comma, //
	TrueC, //
	FalseC, //
	NumC(i32), //
	PlusC, //
	MultC, //
	EqC, //
	IfC, //
	IdC, //
	AppC, //
	FdC,//
	RecC
}