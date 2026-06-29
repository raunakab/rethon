use lexer::lex;

use super::parse_type_declaration_optional;

#[test]
#[ignore]
fn test_type_int() {
    let mut tokens = lex(": int");
    assert!(
        parse_type_declaration_optional(&mut tokens)
            .unwrap()
            .is_some()
    );
}

#[test]
#[ignore]
fn test_type_str() {
    let mut tokens = lex(": str");
    assert!(
        parse_type_declaration_optional(&mut tokens)
            .unwrap()
            .is_some()
    );
}

#[test]
#[ignore]
fn test_type_nested() {
    let mut tokens = lex(": Map[str, int]");
    assert!(
        parse_type_declaration_optional(&mut tokens)
            .unwrap()
            .is_some()
    );
}

#[test]
#[ignore]
fn test_type_absent_ident() {
    let mut tokens = lex("x");
    assert!(
        parse_type_declaration_optional(&mut tokens)
            .unwrap()
            .is_none()
    );
}

#[test]
#[ignore]
fn test_type_absent_number() {
    let mut tokens = lex("42");
    assert!(
        parse_type_declaration_optional(&mut tokens)
            .unwrap()
            .is_none()
    );
}
