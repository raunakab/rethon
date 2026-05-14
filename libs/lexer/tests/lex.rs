use lexer::{Brace, BraceDirection, LexKind, LexType, Res, StringType, lex};

#[rstest::rstest]
// Empty source produces no tokens
#[case("", Ok(vec![]))]
// Single identifier
#[case("x", Ok(vec![LexKind::Normal(LexType::Identifier("x"))]))]
// Keywords
#[case("fn", Ok(vec![LexKind::Normal(LexType::Function)]))]
#[case("mut", Ok(vec![LexKind::Normal(LexType::Mutable)]))]
#[case("scope", Ok(vec![LexKind::Normal(LexType::Scope)]))]
#[case("return", Ok(vec![LexKind::Normal(LexType::Return)]))]
#[case("yield", Ok(vec![LexKind::Normal(LexType::Yield)]))]
#[case("throw", Ok(vec![LexKind::Normal(LexType::Throw)]))]
#[case("otherwise", Ok(vec![LexKind::Normal(LexType::Otherwise)]))]
#[case("not", Ok(vec![LexKind::Normal(LexType::Not)]))]
#[case("and", Ok(vec![LexKind::Normal(LexType::And)]))]
#[case("or", Ok(vec![LexKind::Normal(LexType::Or)]))]
#[case("for", Ok(vec![LexKind::Normal(LexType::For)]))]
#[case("loop", Ok(vec![LexKind::Normal(LexType::Loop)]))]
#[case("if", Ok(vec![LexKind::Normal(LexType::If)]))]
#[case("else", Ok(vec![LexKind::Normal(LexType::Else)]))]
#[case("struct", Ok(vec![LexKind::Normal(LexType::Struct)]))]
#[case("enum", Ok(vec![LexKind::Normal(LexType::Enum)]))]
#[case("panic", Ok(vec![LexKind::Normal(LexType::Panic)]))]
#[case("todo", Ok(vec![LexKind::Normal(LexType::Todo)]))]
#[case("unimplemented", Ok(vec![LexKind::Normal(LexType::Unimplemented)]))]
#[case("true", Ok(vec![LexKind::Normal(LexType::True)]))]
#[case("false", Ok(vec![LexKind::Normal(LexType::False)]))]
// `f` alone is an identifier, not a formatted string prefix
#[case("f", Ok(vec![LexKind::Normal(LexType::Identifier("f"))]))]
// Numbers
#[case("42", Ok(vec![LexKind::Normal(LexType::Number("42"))]))]
#[case("0", Ok(vec![LexKind::Normal(LexType::Number("0"))]))]
#[case("3.14", Ok(vec![LexKind::Normal(LexType::Float("3", Some("14")))]))]
#[case("1.", Ok(vec![LexKind::Normal(LexType::Float("1", None))]))]
#[case("1. 0", Ok(vec![LexKind::Normal(LexType::Float("1", None)), LexKind::Whitespace(1), LexKind::Normal(LexType::Number("0"))]))]
#[case("0 1. 0", Ok(vec![LexKind::Normal(LexType::Number("0")), LexKind::Whitespace(1), LexKind::Normal(LexType::Float("1", None)), LexKind::Whitespace(1), LexKind::Normal(LexType::Number("0"))]))]
// Strings
#[case("\"hello\"", Ok(vec![LexKind::Normal(LexType::String("hello", StringType::Normal))]))]
#[case("\"\"", Ok(vec![LexKind::Normal(LexType::String("", StringType::Normal))]))]
#[case("f\"hi ${name}\"", Ok(vec![LexKind::Normal(LexType::String("hi ${name}", StringType::Formatted))]))]
// Operators
#[case(":=", Ok(vec![LexKind::Normal(LexType::StaticAssignment)]))]
#[case("=", Ok(vec![LexKind::Normal(LexType::Assignment)]))]
#[case("==", Ok(vec![LexKind::Normal(LexType::Equals)]))]
#[case("!", Ok(vec![LexKind::Normal(LexType::Promotion)]))]
#[case("!foo", Ok(vec![LexKind::Normal(LexType::MacroIdentifier("foo"))]))]
#[case("?", Ok(vec![LexKind::Normal(LexType::Coalescence)]))]
#[case("@", Ok(vec![LexKind::Normal(LexType::Ampersand)]))]
#[case(":", Ok(vec![LexKind::Normal(LexType::Colon)]))]
#[case(".", Ok(vec![LexKind::Normal(LexType::Dot)]))]
#[case("..", Ok(vec![LexKind::Normal(LexType::DoubleDot)]))]
#[case("+", Ok(vec![LexKind::Normal(LexType::Plus)]))]
#[case("-", Ok(vec![LexKind::Normal(LexType::Minus)]))]
#[case("--", Ok(vec![LexKind::Normal(LexType::DoubleMinus)]))]
#[case("->", Ok(vec![LexKind::Normal(LexType::Arrow)]))]
#[case("*", Ok(vec![LexKind::Normal(LexType::Asterisk)]))]
#[case("**", Ok(vec![LexKind::Normal(LexType::DoubleAsterisk)]))]
#[case("/", Ok(vec![LexKind::Normal(LexType::Slash)]))]
#[case(";", Ok(vec![LexKind::Normal(LexType::Semicolon)]))]
#[case(",", Ok(vec![LexKind::Normal(LexType::Comma)]))]
#[case("|", Ok(vec![LexKind::Normal(LexType::Pipe)]))]
#[case("|>", Ok(vec![LexKind::Normal(LexType::PipeForward)]))]
#[case("|>>", Ok(vec![LexKind::Normal(LexType::PipeDoubleForward)]))]
#[case(">", Ok(vec![LexKind::Normal(LexType::Greater)]))]
#[case(">>", Ok(vec![LexKind::Normal(LexType::DoubleGreater)]))]
#[case(">=", Ok(vec![LexKind::Normal(LexType::GreaterOrEqual)]))]
#[case("<", Ok(vec![LexKind::Normal(LexType::Lesser)]))]
#[case("<<", Ok(vec![LexKind::Normal(LexType::DoubleLesser)]))]
#[case("<=", Ok(vec![LexKind::Normal(LexType::LesserOrEqual)]))]
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
    LexKind::Normal(LexType::Identifier("a")),
    LexKind::Newline,
    LexKind::Normal(LexType::Identifier("b")),
]))]
#[case("x := 42", Ok(vec![
    LexKind::Normal(LexType::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::StaticAssignment),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Number("42")),
]))]
// Function call with parens
#[case("func()", Ok(vec![
    LexKind::Normal(LexType::Identifier("func")),
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
