use lexer::{StringType, lex};

use crate::Literal;

use super::{parse_ident, parse_literal, parse_literal_optional};

// --- parse_literal_optional ---

#[rstest::rstest]
#[case("true", Some(Literal::True))]
#[case("false", Some(Literal::False))]
#[case("42", Some(Literal::Number("42")))]
#[case("3.", Some(Literal::Float("3", None)))]
#[case("3.14", Some(Literal::Float("3", Some("14"))))]
#[case(r#""hello""#, Some(Literal::String("hello", StringType::Normal)))]
#[case(r#"f"hello""#, Some(Literal::String("hello", StringType::Formatted)))]
#[case("x", None)]
#[case("if", None)]
fn test_literal_optional(#[case] source: &str, #[case] expected: Option<Literal>) {
    let mut tokens = lex(source);
    assert_eq!(parse_literal_optional(&mut tokens).unwrap(), expected);
}

// --- parse_literal ---

#[rstest::rstest]
#[case("true", Literal::True)]
#[case("false", Literal::False)]
#[case("0", Literal::Number("0"))]
#[case("1.0", Literal::Float("1", Some("0")))]
fn test_literal(#[case] source: &str, #[case] expected: Literal) {
    let mut tokens = lex(source);
    assert_eq!(parse_literal(&mut tokens).unwrap(), expected);
}

#[test]
fn test_literal_non_literal_errors() {
    let mut tokens = lex("x");
    assert!(parse_literal(&mut tokens).is_err());
}

// --- parse_ident ---

#[rstest::rstest]
#[case("foo", "foo")]
#[case("bar", "bar")]
#[case("x", "x")]
fn test_ident(#[case] source: &str, #[case] expected: &str) {
    let mut tokens = lex(source);
    assert_eq!(parse_ident(&mut tokens).unwrap(), expected);
}

// --- parse_pattern ---

#[test]
#[ignore]
fn test_pattern_underscore() {
    let mut tokens = lex("_");
    super::parse_pattern(&mut tokens).unwrap();
}

#[test]
#[ignore]
fn test_pattern_ident() {
    let mut tokens = lex("x");
    super::parse_pattern(&mut tokens).unwrap();
}

#[test]
#[ignore]
fn test_pattern_ident_bind() {
    let mut tokens = lex("x @ y");
    super::parse_pattern(&mut tokens).unwrap();
}

#[test]
#[ignore]
fn test_pattern_or() {
    let mut tokens = lex("x | y");
    super::parse_pattern(&mut tokens).unwrap();
}

#[test]
#[ignore]
fn test_pattern_tuple() {
    let mut tokens = lex("(x, y)");
    super::parse_pattern(&mut tokens).unwrap();
}

#[test]
#[ignore]
fn test_pattern_list() {
    let mut tokens = lex("[x, y]");
    super::parse_pattern(&mut tokens).unwrap();
}

#[test]
#[ignore]
fn test_pattern_enum_tuple() {
    let mut tokens = lex("Foo(x, y)");
    super::parse_pattern(&mut tokens).unwrap();
}

#[test]
#[ignore]
fn test_pattern_enum_struct() {
    let mut tokens = lex("Foo { x, y }");
    super::parse_pattern(&mut tokens).unwrap();
}

#[test]
#[ignore]
fn test_pattern_map() {
    let mut tokens = lex("{true: x}");
    super::parse_pattern(&mut tokens).unwrap();
}
