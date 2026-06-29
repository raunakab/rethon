use lexer::lex;

use super::parse_type_declaration_optional;

#[test]
#[ignore]
fn test_type_present() {
    // `: SomeType` — colon followed by a type expression
    let mut tokens = lex(": SomeType");
    let result = parse_type_declaration_optional(&mut tokens).unwrap();
    assert!(result.is_some());
}

#[test]
#[ignore]
fn test_type_absent() {
    // no leading colon — type is optional, returns None
    let mut tokens = lex("x");
    let result = parse_type_declaration_optional(&mut tokens).unwrap();
    assert!(result.is_none());
}

#[test]
#[ignore]
fn test_type_nested() {
    // a more complex type expression
    let mut tokens = lex(": Map[str, int]");
    let result = parse_type_declaration_optional(&mut tokens).unwrap();
    assert!(result.is_some());
}
