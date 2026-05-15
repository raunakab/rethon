use crate::{
    Brace, BraceDirection, Error, Res, StringType, Token,
    s1_segmenter::segment,
    s2_clusterer::{LexKind, cluster},
};

#[rstest::rstest]
#[case("", Ok(vec![]))]
#[case("a", Ok(vec![LexKind::Normal(Token::Identifier("a"))]))]
#[case("ab", Ok(vec![LexKind::Normal(Token::Identifier("ab"))]))]
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
#[case("match", Ok(vec![LexKind::Normal(Token::Match)]))]
#[case("struct", Ok(vec![LexKind::Normal(Token::Struct)]))]
#[case("enum", Ok(vec![LexKind::Normal(Token::Enum)]))]
#[case("panic", Ok(vec![LexKind::Normal(Token::Panic)]))]
#[case("todo", Ok(vec![LexKind::Normal(Token::Todo)]))]
#[case("unimplemented", Ok(vec![LexKind::Normal(Token::Unimplemented)]))]
#[case("true", Ok(vec![LexKind::Normal(Token::True)]))]
#[case("false", Ok(vec![LexKind::Normal(Token::False)]))]
// `f` alone is an identifier, not a formatted string prefix
#[case("f", Ok(vec![LexKind::Normal(Token::Identifier("f"))]))]
#[case("12", Ok(vec![LexKind::Normal(Token::Number("12"))]))]
#[case("12.34", Ok(vec![LexKind::Normal(Token::Float("12", Some("34")))]))]
#[case("12.", Ok(vec![LexKind::Normal(Token::Float("12", None))]))]
#[case("0.5", Ok(vec![LexKind::Normal(Token::Float("0", Some("5")))]))]
#[case(":=", Ok(vec![LexKind::Normal(Token::StaticAssignment)]))]
#[case("=", Ok(vec![LexKind::Normal(Token::Assignment)]))]
#[case("==", Ok(vec![LexKind::Normal(Token::Equals)]))]
#[case("!", Ok(vec![LexKind::Normal(Token::Promotion)]))]
#[case("!hello", Ok(vec![LexKind::Normal(Token::MacroIdentifier("hello"))]))]
#[case("!macrocall", Ok(vec![LexKind::Normal(Token::MacroIdentifier("macrocall"))]))]
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
#[case("\"hello\"", Ok(vec![LexKind::Normal(Token::String("hello", StringType::Normal))]))]
#[case("\"\"", Ok(vec![LexKind::Normal(Token::String("", StringType::Normal))]))]
#[case("f\"hi ${name}\"", Ok(vec![LexKind::Normal(Token::String("hi ${name}", StringType::Formatted))]))]
#[case("f\"value ${name}\"", Ok(vec![LexKind::Normal(Token::String("value ${name}", StringType::Formatted))]))]
// Error cases
#[case("\t", Err(Error::InvalidWhitespace("\t".to_string())))]
#[case("\r", Err(Error::InvalidWhitespace("\r".to_string())))]
#[case("a\tb", Err(Error::InvalidWhitespace("\t".to_string())))]
#[case("\x01", Err(Error::UnknownItem("\x01".to_string())))]
#[case("'", Err(Error::UnknownItem("'".to_string())))]
#[case("\"", Err(Error::UnterminatedString(0)))]
#[case("\"hello", Err(Error::UnterminatedString(0)))]
#[case("x = \"unterminated", Err(Error::UnterminatedString(4)))]
#[case("f\"unterminated ${name}", Err(Error::UnterminatedString(1)))]
// Multi-token cases
#[case("a\nb", Ok(vec![
    LexKind::Normal(Token::Identifier("a")),
    LexKind::Newline,
    LexKind::Normal(Token::Identifier("b")),
]))]
#[case("a\r\nb", Ok(vec![
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
#[case("x = 12.5", Ok(vec![
    LexKind::Normal(Token::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Assignment),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Float("12", Some("5"))),
]))]
#[case("x = \"value\"", Ok(vec![
    LexKind::Normal(Token::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Assignment),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::String("value", StringType::Normal)),
]))]
#[case("a  b", Ok(vec![
    LexKind::Normal(Token::Identifier("a")),
    LexKind::Whitespace(2),
    LexKind::Normal(Token::Identifier("b")),
]))]
#[case("x   =   y", Ok(vec![
    LexKind::Normal(Token::Identifier("x")),
    LexKind::Whitespace(3),
    LexKind::Normal(Token::Assignment),
    LexKind::Whitespace(3),
    LexKind::Normal(Token::Identifier("y")),
]))]
#[case("12abc", Ok(vec![
    LexKind::Normal(Token::Number("12")),
    LexKind::Normal(Token::Identifier("abc")),
]))]
#[case("a .. b", Ok(vec![
    LexKind::Normal(Token::Identifier("a")),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::DoubleDot),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Identifier("b")),
]))]
#[case("x |> f |>> g", Ok(vec![
    LexKind::Normal(Token::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::PipeForward),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Identifier("f")),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::PipeDoubleForward),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Identifier("g")),
]))]
#[case("print(f\"value: ${value}\")", Ok(vec![
    LexKind::Normal(Token::Identifier("print")),
    LexKind::Brace(Brace::Round, BraceDirection::Open),
    LexKind::Normal(Token::String("value: ${value}", StringType::Formatted)),
    LexKind::Brace(Brace::Round, BraceDirection::Close),
]))]
#[case("if true { x } else { y }", Ok(vec![
    LexKind::Normal(Token::If),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::True),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Open),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Close),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Else),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Open),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Identifier("y")),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Close),
]))]
#[case("fn add(x, y) { return x + y; }", Ok(vec![
    LexKind::Normal(Token::Function),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Identifier("add")),
    LexKind::Brace(Brace::Round, BraceDirection::Open),
    LexKind::Normal(Token::Identifier("x")),
    LexKind::Normal(Token::Comma),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Identifier("y")),
    LexKind::Brace(Brace::Round, BraceDirection::Close),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Open),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Return),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Plus),
    LexKind::Whitespace(1),
    LexKind::Normal(Token::Identifier("y")),
    LexKind::Normal(Token::Semicolon),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Close),
]))]
fn test_stage2_tokenization(#[case] source: &str, #[case] expected: Res<Vec<LexKind>>) {
    assert_eq!(
        cluster(segment(source))
            .map(|token| {
                let token = token?;
                Ok(token.kind)
            })
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
