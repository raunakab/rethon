use lexer::{Brace, BraceDirection, LexKind, Res, StringType, TokenType, lex};

#[rstest::rstest]
// Empty source produces no tokens
#[case("", Ok(vec![]))]
// Single identifier
#[case("x", Ok(vec![LexKind::Normal(TokenType::Identifier("x"))]))]
// Keywords
#[case("fn", Ok(vec![LexKind::Normal(TokenType::Function)]))]
#[case("mut", Ok(vec![LexKind::Normal(TokenType::Mutable)]))]
#[case("scope", Ok(vec![LexKind::Normal(TokenType::Scope)]))]
#[case("return", Ok(vec![LexKind::Normal(TokenType::Return)]))]
#[case("yield", Ok(vec![LexKind::Normal(TokenType::Yield)]))]
#[case("throw", Ok(vec![LexKind::Normal(TokenType::Throw)]))]
#[case("otherwise", Ok(vec![LexKind::Normal(TokenType::Otherwise)]))]
#[case("not", Ok(vec![LexKind::Normal(TokenType::Not)]))]
#[case("and", Ok(vec![LexKind::Normal(TokenType::And)]))]
#[case("or", Ok(vec![LexKind::Normal(TokenType::Or)]))]
#[case("for", Ok(vec![LexKind::Normal(TokenType::For)]))]
#[case("loop", Ok(vec![LexKind::Normal(TokenType::Loop)]))]
#[case("if", Ok(vec![LexKind::Normal(TokenType::If)]))]
#[case("else", Ok(vec![LexKind::Normal(TokenType::Else)]))]
#[case("struct", Ok(vec![LexKind::Normal(TokenType::Struct)]))]
#[case("enum", Ok(vec![LexKind::Normal(TokenType::Enum)]))]
#[case("panic", Ok(vec![LexKind::Normal(TokenType::Panic)]))]
#[case("todo", Ok(vec![LexKind::Normal(TokenType::Todo)]))]
#[case("unimplemented", Ok(vec![LexKind::Normal(TokenType::Unimplemented)]))]
#[case("true", Ok(vec![LexKind::Normal(TokenType::True)]))]
#[case("false", Ok(vec![LexKind::Normal(TokenType::False)]))]
// `f` alone is an identifier, not a formatted string prefix
#[case("f", Ok(vec![LexKind::Normal(TokenType::Identifier("f"))]))]
// Numbers
#[case("42", Ok(vec![LexKind::Normal(TokenType::Number("42"))]))]
#[case("0", Ok(vec![LexKind::Normal(TokenType::Number("0"))]))]
#[case("3.14", Ok(vec![LexKind::Normal(TokenType::Float("3", Some("14")))]))]
#[case("1.", Ok(vec![LexKind::Normal(TokenType::Float("1", None))]))]
#[case("1. 0", Ok(vec![LexKind::Normal(TokenType::Float("1", None)), LexKind::Whitespace(1), LexKind::Normal(TokenType::Number("0"))]))]
#[case("0 1. 0", Ok(vec![LexKind::Normal(TokenType::Number("0")), LexKind::Whitespace(1), LexKind::Normal(TokenType::Float("1", None)), LexKind::Whitespace(1), LexKind::Normal(TokenType::Number("0"))]))]
// Strings
#[case("\"hello\"", Ok(vec![LexKind::Normal(TokenType::String("hello", StringType::Normal))]))]
#[case("\"\"", Ok(vec![LexKind::Normal(TokenType::String("", StringType::Normal))]))]
#[case("f\"hi ${name}\"", Ok(vec![LexKind::Normal(TokenType::String("hi ${name}", StringType::Formatted))]))]
// Operators
#[case(":=", Ok(vec![LexKind::Normal(TokenType::StaticAssignment)]))]
#[case("=", Ok(vec![LexKind::Normal(TokenType::Assignment)]))]
#[case("==", Ok(vec![LexKind::Normal(TokenType::Equals)]))]
#[case("!", Ok(vec![LexKind::Normal(TokenType::Promotion)]))]
#[case("!foo", Ok(vec![LexKind::Normal(TokenType::MacroIdentifier("foo"))]))]
#[case("?", Ok(vec![LexKind::Normal(TokenType::Coalescence)]))]
#[case("@", Ok(vec![LexKind::Normal(TokenType::Ampersand)]))]
#[case(":", Ok(vec![LexKind::Normal(TokenType::Colon)]))]
#[case(".", Ok(vec![LexKind::Normal(TokenType::Dot)]))]
#[case("..", Ok(vec![LexKind::Normal(TokenType::DoubleDot)]))]
#[case("+", Ok(vec![LexKind::Normal(TokenType::Plus)]))]
#[case("-", Ok(vec![LexKind::Normal(TokenType::Minus)]))]
#[case("--", Ok(vec![LexKind::Normal(TokenType::DoubleMinus)]))]
#[case("->", Ok(vec![LexKind::Normal(TokenType::Arrow)]))]
#[case("*", Ok(vec![LexKind::Normal(TokenType::Asterisk)]))]
#[case("**", Ok(vec![LexKind::Normal(TokenType::DoubleAsterisk)]))]
#[case("/", Ok(vec![LexKind::Normal(TokenType::Slash)]))]
#[case(";", Ok(vec![LexKind::Normal(TokenType::Semicolon)]))]
#[case(",", Ok(vec![LexKind::Normal(TokenType::Comma)]))]
#[case("|", Ok(vec![LexKind::Normal(TokenType::Pipe)]))]
#[case("|>", Ok(vec![LexKind::Normal(TokenType::PipeForward)]))]
#[case("|>>", Ok(vec![LexKind::Normal(TokenType::PipeDoubleForward)]))]
#[case(">", Ok(vec![LexKind::Normal(TokenType::Greater)]))]
#[case(">>", Ok(vec![LexKind::Normal(TokenType::DoubleGreater)]))]
#[case(">=", Ok(vec![LexKind::Normal(TokenType::GreaterOrEqual)]))]
#[case("<", Ok(vec![LexKind::Normal(TokenType::Lesser)]))]
#[case("<<", Ok(vec![LexKind::Normal(TokenType::DoubleLesser)]))]
#[case("<=", Ok(vec![LexKind::Normal(TokenType::LesserOrEqual)]))]
// Braces
#[case("(", Ok(vec![LexKind::Brace(Brace::Round, BraceDirection::Open)]))]
#[case(")", Ok(vec![LexKind::Brace(Brace::Round, BraceDirection::Close)]))]
#[case("[", Ok(vec![LexKind::Brace(Brace::Square, BraceDirection::Open)]))]
#[case("]", Ok(vec![LexKind::Brace(Brace::Square, BraceDirection::Close)]))]
#[case("{", Ok(vec![LexKind::Brace(Brace::Curly, BraceDirection::Open)]))]
#[case("}", Ok(vec![LexKind::Brace(Brace::Curly, BraceDirection::Close)]))]
// Whitespace and newlines
#[case("\n", Ok(vec![LexKind::Newline]))]
#[case("a\nb", Ok(vec![
    LexKind::Normal(TokenType::Identifier("a")),
    LexKind::Newline,
    LexKind::Normal(TokenType::Identifier("b")),
]))]
#[case("x := 42", Ok(vec![
    LexKind::Normal(TokenType::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::StaticAssignment),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Number("42")),
]))]
// Function call with parens
#[case("func()", Ok(vec![
    LexKind::Normal(TokenType::Identifier("func")),
    LexKind::Brace(Brace::Round, BraceDirection::Open),
    LexKind::Brace(Brace::Round, BraceDirection::Close),
]))]
// Error cases
#[case("\t", Err(lexer::Error::InvalidWhitespace("\t".to_string())))]
#[case("\"unclosed", Err(lexer::Error::UnterminatedString(0)))]
#[case("'", Err(lexer::Error::UnknownToken("'".to_string())))]
#[case("\x01", Err(lexer::Error::UnknownToken("\x01".to_string())))]
fn test_lex(#[case] source: &str, #[case] expected: Res<Vec<LexKind>>) {
    assert_eq!(
        lex(source)
            .map(|r| r.map(|t| t.kind))
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
