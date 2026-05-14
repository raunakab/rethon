use crate::{
    Brace, BraceDirection, Error, LexKind, LexType, Res, StringType, s1_segmenter::segment,
    s2_clusterer::cluster,
};

#[rstest::rstest]
#[case("", Ok(vec![]))]
#[case("a", Ok(vec![LexKind::Normal(LexType::Identifier("a"))]))]
#[case("ab", Ok(vec![LexKind::Normal(LexType::Identifier("ab"))]))]
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
#[case("12", Ok(vec![LexKind::Normal(LexType::Number("12"))]))]
#[case("12.34", Ok(vec![LexKind::Normal(LexType::Float("12", Some("34")))]))]
#[case("12.", Ok(vec![LexKind::Normal(LexType::Float("12", None))]))]
#[case("0.5", Ok(vec![LexKind::Normal(LexType::Float("0", Some("5")))]))]
#[case(":=", Ok(vec![LexKind::Normal(LexType::StaticAssignment)]))]
#[case("=", Ok(vec![LexKind::Normal(LexType::Assignment)]))]
#[case("==", Ok(vec![LexKind::Normal(LexType::Equals)]))]
#[case("!", Ok(vec![LexKind::Normal(LexType::Promotion)]))]
#[case("!hello", Ok(vec![LexKind::Normal(LexType::MacroIdentifier("hello"))]))]
#[case("!macrocall", Ok(vec![LexKind::Normal(LexType::MacroIdentifier("macrocall"))]))]
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
#[case("\"hello\"", Ok(vec![LexKind::Normal(LexType::String("hello", StringType::Normal))]))]
#[case("\"\"", Ok(vec![LexKind::Normal(LexType::String("", StringType::Normal))]))]
#[case("f\"hi ${name}\"", Ok(vec![LexKind::Normal(LexType::String("hi ${name}", StringType::Formatted))]))]
#[case("f\"value ${name}\"", Ok(vec![LexKind::Normal(LexType::String("value ${name}", StringType::Formatted))]))]
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
    LexKind::Normal(LexType::Identifier("a")),
    LexKind::Newline,
    LexKind::Normal(LexType::Identifier("b")),
]))]
#[case("a\r\nb", Ok(vec![
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
#[case("x = 12.5", Ok(vec![
    LexKind::Normal(LexType::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Assignment),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Float("12", Some("5"))),
]))]
#[case("x = \"value\"", Ok(vec![
    LexKind::Normal(LexType::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Assignment),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::String("value", StringType::Normal)),
]))]
#[case("a  b", Ok(vec![
    LexKind::Normal(LexType::Identifier("a")),
    LexKind::Whitespace(2),
    LexKind::Normal(LexType::Identifier("b")),
]))]
#[case("x   =   y", Ok(vec![
    LexKind::Normal(LexType::Identifier("x")),
    LexKind::Whitespace(3),
    LexKind::Normal(LexType::Assignment),
    LexKind::Whitespace(3),
    LexKind::Normal(LexType::Identifier("y")),
]))]
#[case("12abc", Ok(vec![
    LexKind::Normal(LexType::Number("12")),
    LexKind::Normal(LexType::Identifier("abc")),
]))]
#[case("a .. b", Ok(vec![
    LexKind::Normal(LexType::Identifier("a")),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::DoubleDot),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Identifier("b")),
]))]
#[case("x |> f |>> g", Ok(vec![
    LexKind::Normal(LexType::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::PipeForward),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Identifier("f")),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::PipeDoubleForward),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Identifier("g")),
]))]
#[case("print(f\"value: ${value}\")", Ok(vec![
    LexKind::Normal(LexType::Identifier("print")),
    LexKind::Brace(Brace::Round, BraceDirection::Open),
    LexKind::Normal(LexType::String("value: ${value}", StringType::Formatted)),
    LexKind::Brace(Brace::Round, BraceDirection::Close),
]))]
#[case("if true { x } else { y }", Ok(vec![
    LexKind::Normal(LexType::If),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::True),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Open),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Close),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Else),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Open),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Identifier("y")),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Close),
]))]
#[case("fn add(x, y) { return x + y; }", Ok(vec![
    LexKind::Normal(LexType::Function),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Identifier("add")),
    LexKind::Brace(Brace::Round, BraceDirection::Open),
    LexKind::Normal(LexType::Identifier("x")),
    LexKind::Normal(LexType::Comma),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Identifier("y")),
    LexKind::Brace(Brace::Round, BraceDirection::Close),
    LexKind::Whitespace(1),
    LexKind::Brace(Brace::Curly, BraceDirection::Open),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Return),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Identifier("x")),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Plus),
    LexKind::Whitespace(1),
    LexKind::Normal(LexType::Identifier("y")),
    LexKind::Normal(LexType::Semicolon),
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
