use crate::{
    Brace, BraceDirection, Error, LexKind, Res, StringType, TokenType,
    lexer_s1::tokenize as s1_tokenize, lexer_s2::tokenize,
};

#[rstest::rstest]
#[case("", Ok(vec![]))]
#[case("a", Ok(vec![LexKind::Normal(TokenType::Identifier("a"))]))]
#[case("ab", Ok(vec![LexKind::Normal(TokenType::Identifier("ab"))]))]
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
#[case("12", Ok(vec![LexKind::Normal(TokenType::Number("12"))]))]
#[case("12.34", Ok(vec![LexKind::Normal(TokenType::Float("12", Some("34")))]))]
#[case("12.", Ok(vec![LexKind::Normal(TokenType::Float("12", None))]))]
#[case("0.5", Ok(vec![LexKind::Normal(TokenType::Float("0", Some("5")))]))]
#[case(":=", Ok(vec![LexKind::Normal(TokenType::StaticAssignment)]))]
#[case("=", Ok(vec![LexKind::Normal(TokenType::Assignment)]))]
#[case("==", Ok(vec![LexKind::Normal(TokenType::Equals)]))]
#[case("!", Ok(vec![LexKind::Normal(TokenType::Promotion)]))]
#[case("!hello", Ok(vec![LexKind::Normal(TokenType::MacroIdentifier("hello"))]))]
#[case("!macrocall", Ok(vec![LexKind::Normal(TokenType::MacroIdentifier("macrocall"))]))]
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
#[case("(", Ok(vec![LexKind::Brace(Brace::Round, BraceDirection::Open)]))]
#[case(")", Ok(vec![LexKind::Brace(Brace::Round, BraceDirection::Close)]))]
#[case("[", Ok(vec![LexKind::Brace(Brace::Square, BraceDirection::Open)]))]
#[case("]", Ok(vec![LexKind::Brace(Brace::Square, BraceDirection::Close)]))]
#[case("{", Ok(vec![LexKind::Brace(Brace::Curly, BraceDirection::Open)]))]
#[case("}", Ok(vec![LexKind::Brace(Brace::Curly, BraceDirection::Close)]))]
#[case("\n", Ok(vec![LexKind::Newline]))]
#[case("\n\n", Ok(vec![LexKind::Newline, LexKind::Newline]))]
#[case(" ", Ok(vec![LexKind::Whitespace(1)]))]
#[case("  ", Ok(vec![LexKind::Whitespace(2)]))]
#[case("   ", Ok(vec![LexKind::Whitespace(3)]))]
#[case("\"hello\"", Ok(vec![LexKind::Normal(TokenType::String("hello", StringType::Normal))]))]
#[case("\"\"", Ok(vec![LexKind::Normal(TokenType::String("", StringType::Normal))]))]
#[case("f\"hi ${name}\"", Ok(vec![LexKind::Normal(TokenType::String("hi ${name}", StringType::Formatted))]))]
#[case("f\"value ${name}\"", Ok(vec![LexKind::Normal(TokenType::String("value ${name}", StringType::Formatted))]))]
// Error cases
#[case("\t", Err(Error::InvalidWhitespace("\t".to_string())))]
#[case("\r", Err(Error::InvalidWhitespace("\r".to_string())))]
#[case("a\tb", Err(Error::InvalidWhitespace("\t".to_string())))]
#[case("\x01", Err(Error::UnknownToken("\x01".to_string())))]
#[case("'", Err(Error::UnknownToken("'".to_string())))]
#[case("\"", Err(Error::UnterminatedString(0)))]
#[case("\"hello", Err(Error::UnterminatedString(0)))]
#[case("x = \"unterminated", Err(Error::UnterminatedString(4)))]
#[case("f\"unterminated ${name}", Err(Error::UnterminatedString(1)))]
// Multi-token cases
#[case("a\nb", Ok(vec![
    LexKind::Normal(TokenType::Identifier("a")),
    LexKind::Newline,
    LexKind::Normal(TokenType::Identifier("b")),
]))]
#[case("a\r\nb", Ok(vec![
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
#[case("x = 12.5", Ok(vec![
    LexKind::Normal(TokenType::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Assignment),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Float("12", Some("5"))),
]))]
#[case("x = \"value\"", Ok(vec![
    LexKind::Normal(TokenType::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Assignment),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::String("value", StringType::Normal)),
]))]
#[case("a  b", Ok(vec![
    LexKind::Normal(TokenType::Identifier("a")),
    LexKind::Whitespace(2),
    LexKind::Normal(TokenType::Identifier("b")),
]))]
#[case("x   =   y", Ok(vec![
    LexKind::Normal(TokenType::Identifier("x")),
    LexKind::Whitespace(3),
    LexKind::Normal(TokenType::Assignment),
    LexKind::Whitespace(3),
    LexKind::Normal(TokenType::Identifier("y")),
]))]
#[case("12abc", Ok(vec![
    LexKind::Normal(TokenType::Number("12")),
    LexKind::Normal(TokenType::Identifier("abc")),
]))]
#[case("a .. b", Ok(vec![
    LexKind::Normal(TokenType::Identifier("a")),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::DoubleDot),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Identifier("b")),
]))]
#[case("x |> f |>> g", Ok(vec![
    LexKind::Normal(TokenType::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::PipeForward),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Identifier("f")),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::PipeDoubleForward),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Identifier("g")),
]))]
#[case("print(f\"value: ${value}\")", Ok(vec![
    LexKind::Normal(TokenType::Identifier("print")),
    LexKind::Brace(Brace::Round, BraceDirection::Open),
    LexKind::Normal(TokenType::String("value: ${value}", StringType::Formatted)),
    LexKind::Brace(Brace::Round, BraceDirection::Close),
]))]
#[case("if true { x } else { y }", Ok(vec![
    LexKind::Normal(TokenType::If),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::True),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Open),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Close),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Else),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Open),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Identifier("y")),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Close),
]))]
#[case("fn add(x, y) { return x + y; }", Ok(vec![
    LexKind::Normal(TokenType::Function),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Identifier("add")),
    LexKind::Brace(Brace::Round, BraceDirection::Open),
    LexKind::Normal(TokenType::Identifier("x")),
    LexKind::Normal(TokenType::Comma),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Identifier("y")),
    LexKind::Brace(Brace::Round, BraceDirection::Close),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Open),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Return),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Plus),
    LexKind::Whitespace(1),
    LexKind::Normal(TokenType::Identifier("y")),
    LexKind::Normal(TokenType::Semicolon),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Close),
]))]
fn test_stage2_tokenization(#[case] source: &str, #[case] expected: Res<Vec<LexKind>>) {
    assert_eq!(
        tokenize(s1_tokenize(source))
            .map(|token| {
                let token = token?;
                Ok(token.kind)
            })
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
