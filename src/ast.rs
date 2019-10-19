#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Empty,
    Char(char),
    Or(Box<Expr>, Box<Expr>),
    Concat(Box<Expr>, Box<Expr>),
    Loop(Box<Expr>),
}
