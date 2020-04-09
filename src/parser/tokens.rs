use nom::{
    IResult,
    sequence::{
        preceded,
    },
    character::complete::{
        multispace0,
    },
    bytes::complete::tag
};

// TODO Make `token` function private (see below)
pub fn token<'a, F, O>(parser: F) -> impl Fn(&'a str) -> IResult<&'a str, O>
where F: Fn(&'a str) -> IResult<&'a str, O> {
    preceded(multispace0, parser)
}

macro_rules! reserved {
    ($lexeme:ident, $lexeme_str:literal) => {
        pub fn $lexeme<'a>(input: &'a str) ->
        IResult<&'a str, &'a str> {
            token(tag($lexeme_str))(input)
        }
    };
}

reserved!(comma, ",");
reserved!(plus, "+");
reserved!(minus, "-");
reserved!(star, "*");
reserved!(slash, "/");
reserved!(modulo, "%");
reserved!(and, "and");
reserved!(or, "or");
reserved!(xor, "xor");
reserved!(not, "not");
reserved!(true_val, "true");
reserved!(false_val, "false");
reserved!(equal, "==");
reserved!(not_equal, "/=");
reserved!(less_than, "<");
reserved!(greater_than, ">");
reserved!(less_than_equal, "<=");
reserved!(greater_than_equal, ">=");
reserved!(left_paren, "(");
reserved!(right_paren, ")");
reserved!(if_, "if");
reserved!(then, "then");
reserved!(q_mark, "?");
reserved!(else_, "else");
reserved!(colon, ":");

// TODO Create literal parser here and make `token` function private

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn token_parser_test() {
        let parser = token(tag("abc"));
        assert_eq!(parser(" \t\nabc"), Ok(("", "abc")));
    }
    
    parser_test!(comma_test (comma): "," => ",");
    parser_test!(plus_test (plus): "+" => "+");
    parser_test!(minus_test (minus): "-" => "-");
    parser_test!(star_test (star): "*" => "*");
    parser_test!(slash_test (slash): "/" => "/");
    parser_test!(modulo_test (modulo): "%" => "%");
    parser_test!(and_test (and): "and" => "and");
    parser_test!(or_test (or): "or" => "or");
    parser_test!(xor_test (xor): "xor" => "xor");
    parser_test!(not_test (not): "not" => "not");
    parser_test!(true_test (true_val): "true" => "true");
    parser_test!(false_test (false_val): "false" => "false");
    parser_test!(left_paren_test (left_paren): "(" => "(");
    parser_test!(right_paren_test (right_paren): ")" => ")");
    parser_test!(if_test (if_): "if" => "if");
    parser_test!(then_test (then): "then" => "then");
    parser_test!(q_mark_test (q_mark): "?" => "?");
    parser_test!(else_test (else_): "else" => "else");
    parser_test!(colon_test (colon): ":" => ":");
    
    // Use find and replace
    // Find: reserved!\(([a-z_]+), ("[^"]+")\);
    // Replace: parser_test!(\1_test (\1): \2 => \2);
    
}