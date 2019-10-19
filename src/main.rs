#![feature(maybe_uninit)]

mod ast;
mod interpreter;
mod parser;

#[test]
fn test() {
    use interpreter::match_expr;
    use parser::parse_expr;

    assert_eq!(match_expr(&parse_expr("").unwrap(), ""), true);
    assert_eq!(match_expr(&parse_expr("").unwrap(), "a"), false);

    assert_eq!(match_expr(&parse_expr("a").unwrap(), "a"), true);
    assert_eq!(match_expr(&parse_expr("a").unwrap(), ""), false);
    assert_eq!(match_expr(&parse_expr("a").unwrap(), "b"), false);
    assert_eq!(match_expr(&parse_expr("a").unwrap(), "aa"), false);

    assert_eq!(match_expr(&parse_expr("a|b").unwrap(), "a"), true);
    assert_eq!(match_expr(&parse_expr("a|b").unwrap(), "b"), true);
    assert_eq!(match_expr(&parse_expr("a|b").unwrap(), "c"), false);

    assert_eq!(match_expr(&parse_expr("a*").unwrap(), ""), true);
    assert_eq!(match_expr(&parse_expr("a*").unwrap(), "aaaaa"), true);
    assert_eq!(match_expr(&parse_expr("a*").unwrap(), "aaaab"), false);

    assert_eq!(match_expr(&parse_expr("a*b").unwrap(), "ab"), true);

    assert_eq!(
        match_expr(&parse_expr("(ab|c)*").unwrap(), "abababcabcccab"),
        true
    );
    assert_eq!(
        match_expr(&parse_expr("(ab|c)*").unwrap(), "abababcabcccba"),
        false
    );

    assert_eq!(match_expr(&parse_expr("a+").unwrap(), "aaa"), true);
    assert_eq!(match_expr(&parse_expr("a+").unwrap(), ""), false);

    assert_eq!(match_expr(&parse_expr("a?").unwrap(), ""), true);
    assert_eq!(match_expr(&parse_expr("a?").unwrap(), "a"), true);
    assert_eq!(match_expr(&parse_expr("a?").unwrap(), "aa"), false);
}

fn main() {}
