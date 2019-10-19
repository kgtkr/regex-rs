use crate::ast::Expr;
use combine::char::{char, lower};
use combine::Parser;
use combine::Stream;
use combine::{chainl1, choice, eof, optional, parser, value};

pub fn parse_expr(s: &str) -> Option<Expr> {
    (expr(), eof()).parse(s).map(|((x, _), _)| x).ok()
}

#[test]
fn test() {
    assert_eq!(parse_expr(""), Some(Expr::Empty));
    assert_eq!(
        parse_expr("||"),
        Some(Expr::Or(
            Box::new(Expr::Or(Box::new(Expr::Empty), Box::new(Expr::Empty))),
            Box::new(Expr::Empty)
        ))
    );
    assert_eq!(parse_expr("((()))"), Some(Expr::Empty));
    assert_eq!(
        parse_expr("a*bc"),
        Some(Expr::Concat(
            Box::new(Expr::Concat(
                Box::new(Expr::Loop(Box::new(Expr::Char('a')))),
                Box::new(Expr::Char('b'))
            )),
            Box::new(Expr::Char('c'))
        ))
    );
}

parser! {
    fn expr[Input]()(Input) -> Expr
    where [Input: Stream<Token = char>]
    {
        chainl1(
            expr2(),
            char('|').map(|_| |a, b| Expr::Or(Box::new(a), Box::new(b))),
        )
    }
}

parser! {
    fn expr2[Input]()(Input) -> Expr
    where [Input: Stream<Token = char>]
    {
        choice((chainl1(expr1(),value(()).map(|_||a,b|Expr::Concat(Box::new(a), Box::new(b)))), value(Expr::Empty)))
    }
}

parser! {
    fn expr1[Input]()(Input) -> Expr
    where [Input: Stream<Token = char>]
    {
        (term(), optional(choice((char('*'),char('+'),char('?'))))).map(|(x, m)| {
            match m{
                Some('*') => Expr::Loop(Box::new(x)),
                Some('+') => Expr::Concat(Box::new(x.clone()),Box::new(Expr::Loop(Box::new(x)))),
                Some('?') => Expr::Or(Box::new(x),Box::new(Expr::Empty)),
                _ => x
            }
        })
    }
}

parser! {
    fn term[Input]()(Input) -> Expr
    where [Input: Stream<Token = char>]
    {
        choice((
            (char('('), expr(), char(')')).map(|(_,x,_)| x),
            lower().map(Expr::Char),
        ))
    }
}
