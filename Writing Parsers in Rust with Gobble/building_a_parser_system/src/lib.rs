pub mod charbool;
pub mod combi;
pub mod err;
pub mod iter;
pub mod parser;

pub use charbool::*;
pub use combi::*;
pub use err::*;
pub use parser::*;
pub use iter::*;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plus() {
        let p = ("(", "abcde".plus(), ")").map(|(_, w, _)| w);
        assert_eq!(p.parse_s("(abbe)ere"), Ok("abbe".to_string()));
        assert!(p.parse_s("(abbg)").is_err());
        assert_eq!(
            p.parse_s("(abbg)"),
            Err(PErr {
                line: 0,
                col: 4,
                index: Some(4),
                found: "g",
                exp: Expected::Str(")"),
                child: None,
                is_break: false,
            })
        )
    }

    fn c_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'B')
    }
    fn p_bool() -> impl Parser<Out = bool> {
        or("true".map(|_| true), "false".map(|_| false))
    }

    //Recursive parsers must be constant types
    fn r_brackets<'a>(i: &PIter<'a>) -> ParseRes<'a, (usize, String)> {
        //use map to match types
        or(
            ("(", r_brackets, ")").map(|(_, (n, s), _)| (n + 1, s)),
            c_alpha.plus().map(|s| (0, s)),
        )
        .parse(i)
    }

    #[test]
    fn test_or_and_map() {
        let p = (p_bool(), "=", p_bool()).map(|(a, _, b)| (a, b));
        assert_eq!(p.parse_s("true=false"), Ok((true, false)));
        assert!(p.parse_s("true*false").is_err());
    }

    #[test]
    fn test_recursion() {
        assert_eq!(r_brackets.parse_s("(((cat)))"), Ok((3, "cat".to_string())));
        assert!(r_brackets.parse_s("((cat").is_err());
    }
}
