use lexer::{Brace, BraceDirection, LexKind, Res, StringType, Token, lex};

#[rstest::rstest]
// Empty source produces no tokens
#[case("", Ok(vec![]))]
// Single identifier
#[case("x", Ok(vec![LexKind::Normal(Token::Identifier("x"))]))]
// Keywords
#[case("fn", Ok(vec![LexKind::Normal(Token::Function)]))]
#[case("mut", Ok(vec![LexKind::Normal(Token::Mutable)]))]
#[case("scope", Ok(vec![LexKind::Normal(Token::Scope)]))]
#[case("return", Ok(vec![LexKind::Normal(Token::Return)]))]
#[case("yield", Ok(vec![LexKind::Normal(Token::Yield)]))]
#[case("throw", Ok(vec![LexKind::Normal(Token::Throw)]))]
#[case("otherwise", Ok(vec![LexKind::Normal(Token::Otherwise)]))]
#[case("not", Ok(vec![LexKind::Normal(Token::Not)]))]
#[case("and", Ok(vec![LexKind::Normal(Token::And)]))]
#[case("or", Ok(vec![LexKind::Normal(Token::Or)]))]
#[case("for", Ok(vec![LexKind::Normal(Token::For)]))]
#[case("loop", Ok(vec![LexKind::Normal(Token::Loop)]))]
#[case("if", Ok(vec![LexKind::Normal(Token::If)]))]
#[case("else", Ok(vec![LexKind::Normal(Token::Else)]))]
#[case("struct", Ok(vec![LexKind::Normal(Token::Struct)]))]
#[case("enum", Ok(vec![LexKind::Normal(Token::Enum)]))]
#[case("panic", Ok(vec![LexKind::Normal(Token::Panic)]))]
#[case("todo", Ok(vec![LexKind::Normal(Token::Todo)]))]
#[case("unimplemented", Ok(vec![LexKind::Normal(Token::Unimplemented)]))]
#[case("true", Ok(vec![LexKind::Normal(Token::True)]))]
#[case("false", Ok(vec![LexKind::Normal(Token::False)]))]
// `f` alone is an identifier, not a formatted string prefix
#[case("f", Ok(vec![LexKind::Normal(Token::Identifier("f"))]))]
// Numbers
#[case("42", Ok(vec![LexKind::Normal(Token::Number("42"))]))]
#[case("0", Ok(vec![LexKind::Normal(Token::Number("0"))]))]
#[case("3.14", Ok(vec![LexKind::Normal(Token::Float("3", Some("14")))]))]
#[case("1.", Ok(vec![LexKind::Normal(Token::Float("1", None))]))]
#[case("1. 0", Ok(vec![LexKind::Normal(Token::Float("1", None)), LexKind::Whitespace(1), LexKind::Normal(Token::Number("0"))]))]
#[case("0 1. 0", Ok(vec![LexKind::Normal(Token::Number("0")), LexKind::Whitespace(1), LexKind::Normal(Token::Float("1", None)), LexKind::Whitespace(1), LexKind::Normal(Token::Number("0"))]))]
// Strings
#[case("\"hello\"", Ok(vec![LexKind::Normal(Token::String("hello", StringType::Normal))]))]
#[case("\"\"", Ok(vec![LexKind::Normal(Token::String("", StringType::Normal))]))]
#[case("f\"hi ${name}\"", Ok(vec![LexKind::Normal(Token::String("hi ${name}", StringType::Formatted))]))]
// Operators
#[case(":=", Ok(vec![LexKind::Normal(Token::StaticAssignment)]))]
#[case("=", Ok(vec![LexKind::Normal(Token::Assignment)]))]
#[case("==", Ok(vec![LexKind::Normal(Token::Equals)]))]
#[case("!", Ok(vec![LexKind::Normal(Token::Promotion)]))]
#[case("!foo", Ok(vec![LexKind::Normal(Token::MacroIdentifier("foo"))]))]
#[case("?", Ok(vec![LexKind::Normal(Token::Coalescence)]))]
#[case("@", Ok(vec![LexKind::Normal(Token::Ampersand)]))]
#[case(":", Ok(vec![LexKind::Normal(Token::Colon)]))]
#[case(".", Ok(vec![LexKind::Normal(Token::Dot)]))]
#[case("..", Ok(vec![LexKind::Normal(Token::DoubleDot)]))]
#[case("+", Ok(vec![LexKind::Normal(Token::Plus)]))]
#[case("-", Ok(vec![LexKind::Normal(Token::Minus)]))]
#[case("--", Ok(vec![LexKind::Normal(Token::DoubleMinus)]))]
#[case("->", Ok(vec![LexKind::Normal(Token::Arrow)]))]
#[case("*", Ok(vec![LexKind::Normal(Token::Asterisk)]))]
#[case("**", Ok(vec![LexKind::Normal(Token::DoubleAsterisk)]))]
#[case("/", Ok(vec![LexKind::Normal(Token::Slash)]))]
#[case(";", Ok(vec![LexKind::Normal(Token::Semicolon)]))]
#[case(",", Ok(vec![LexKind::Normal(Token::Comma)]))]
#[case("|", Ok(vec![LexKind::Normal(Token::Pipe)]))]
#[case("|>", Ok(vec![LexKind::Normal(Token::PipeForward)]))]
#[case("|>>", Ok(vec![LexKind::Normal(Token::PipeDoubleForward)]))]
#[case(">", Ok(vec![LexKind::Normal(Token::Greater)]))]
#[case(">>", Ok(vec![LexKind::Normal(Token::DoubleGreater)]))]
#[case(">=", Ok(vec![LexKind::Normal(Token::GreaterOrEqual)]))]
#[case("<", Ok(vec![LexKind::Normal(Token::Lesser)]))]
#[case("<<", Ok(vec![LexKind::Normal(Token::DoubleLesser)]))]
#[case("<=", Ok(vec![LexKind::Normal(Token::LesserOrEqual)]))]
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
    LexKind::Normal(Token::Identifier("a")),
    LexKind::Newline,
    LexKind::Normal(Token::Identifier("b")),
]))]
#[case("x := 42", Ok(vec![
    LexKind::Normal(Token::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::StaticAssignment),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Number("42")),
]))]
// Function call with parens
#[case("func()", Ok(vec![
    LexKind::Normal(Token::Identifier("func")),
    LexKind::Brace(Brace::Round, BraceDirection::Open),
    LexKind::Brace(Brace::Round, BraceDirection::Close),
]))]
// Error cases
#[case("\t", Err(lexer::Error::InvalidWhitespace("\t".to_string())))]
#[case("\"unclosed", Err(lexer::Error::UnterminatedString(0)))]
#[case("'", Err(lexer::Error::UnknownItem("'".to_string())))]
#[case("\x01", Err(lexer::Error::UnknownItem("\x01".to_string())))]
fn test_lex(#[case] source: &str, #[case] expected: Res<Vec<LexKind>>) {
    assert_eq!(
        lex(source)
            .map(|r| r.map(|t| t.kind))
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
