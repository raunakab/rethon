use lexer::{Brace, BraceDirection, L2TokenType, Res, StringType, TokenType, lex};

#[rstest::rstest]
// Empty source produces no tokens
#[case("", Ok(vec![]))]
// Simple identifier
#[case("x", Ok(vec![L2TokenType::Normal(TokenType::Identifier("x"))]))]
// Keyword
#[case("fn", Ok(vec![L2TokenType::Normal(TokenType::Function)]))]
// Assignment expression
#[case("x := 42", Ok(vec![
    L2TokenType::Normal(TokenType::Identifier("x")),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::StaticAssignment),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Number("42")),
]))]
// Function call with parens
#[case("func()", Ok(vec![
    L2TokenType::Normal(TokenType::Identifier("func")),
    L2TokenType::Brace(Brace::Round, BraceDirection::Open),
    L2TokenType::Brace(Brace::Round, BraceDirection::Close),
]))]
// String literal
#[case("\"hello\"", Ok(vec![
    L2TokenType::Normal(TokenType::String("hello", StringType::Normal)),
]))]
// Formatted string
#[case("f\"hi ${name}\"", Ok(vec![
    L2TokenType::Normal(TokenType::String("hi ${name}", StringType::Formatted)),
]))]
// Newline
#[case("a\nb", Ok(vec![
    L2TokenType::Normal(TokenType::Identifier("a")),
    L2TokenType::Newline,
    L2TokenType::Normal(TokenType::Identifier("b")),
]))]
// Tab is invalid
#[case("\t", Err(lexer::Error::InvalidWhitespace("\t".to_string())))]
// Unterminated string
#[case("\"unclosed", Err(lexer::Error::UnterminatedString(0)))]
fn test_lex(#[case] source: &str, #[case] expected: Res<Vec<L2TokenType>>) {
    assert_eq!(
        lex(source)
            .map(|r| r.map(|t| t.token_type))
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
