use crate::ast::Expr;

pub fn match_expr(pat: &Expr, s: &str) -> bool {
    match_helper(&s.chars().collect(), 0, pat) == Some(s.len())
}

fn match_helper(s: &Vec<char>, i: usize, pat: &Expr) -> Option<usize> {
    match pat {
        Expr::Empty => Some(i),
        Expr::Char(c) => {
            if s.get(i) == Some(c) {
                Some(i + 1)
            } else {
                None
            }
        }
        Expr::Or(a, b) => match (match_helper(s, i, a), match_helper(s, i, b)) {
            (Some(i), _) => Some(i),
            (_, Some(i)) => Some(i),
            (None, None) => None,
        },
        Expr::Concat(a, b) => match match_helper(s, i, a) {
            Some(i) => match_helper(s, i, b),
            None => None,
        },
        Expr::Loop(pat) => {
            let mut i = i;
            while let Some(j) = match_helper(s, i, pat) {
                i = j;
            }
            Some(i)
        }
    }
}
