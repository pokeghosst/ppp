#[derive(Debug)]
pub enum Statement {
    VarDecl(String, Expr),
    ValDecl(String, Expr),
    Assign(String, Expr),
    Print(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Variable(String),
    StringLiteral(String),
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
