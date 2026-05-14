use crate::{
    Brace, BraceDirection, Error, Res, StringType, TokenType,
    lexer_stage1::l1_tokenize,
    lexer_stage2::{L2TokenType, l2_tokenize},
};

#[rstest::rstest]
#[case("", Ok(vec![]))]
#[case("a", Ok(vec![L2TokenType::Normal(TokenType::Identifier("a"))]))]
#[case("ab", Ok(vec![L2TokenType::Normal(TokenType::Identifier("ab"))]))]
#[case("fn", Ok(vec![L2TokenType::Normal(TokenType::Function)]))]
#[case("return", Ok(vec![L2TokenType::Normal(TokenType::Return)]))]
#[case("if", Ok(vec![L2TokenType::Normal(TokenType::If)]))]
#[case("else", Ok(vec![L2TokenType::Normal(TokenType::Else)]))]
#[case("true", Ok(vec![L2TokenType::Normal(TokenType::True)]))]
#[case("false", Ok(vec![L2TokenType::Normal(TokenType::False)]))]
#[case("12", Ok(vec![L2TokenType::Normal(TokenType::Number("12"))]))]
#[case("12.34", Ok(vec![L2TokenType::Normal(TokenType::Float("12", Some("34")))]))]
#[case("12.", Ok(vec![L2TokenType::Normal(TokenType::Float("12", None))]))]
#[case("0.5", Ok(vec![L2TokenType::Normal(TokenType::Float("0", Some("5")))]))]
#[case("==", Ok(vec![L2TokenType::Normal(TokenType::Equals)]))]
#[case("=", Ok(vec![L2TokenType::Normal(TokenType::Assignment)]))]
#[case(":=", Ok(vec![L2TokenType::Normal(TokenType::StaticAssignment)]))]
#[case("..", Ok(vec![L2TokenType::Normal(TokenType::DoubleDot)]))]
#[case(".", Ok(vec![L2TokenType::Normal(TokenType::Dot)]))]
#[case("--", Ok(vec![L2TokenType::Normal(TokenType::DoubleMinus)]))]
#[case("->", Ok(vec![L2TokenType::Normal(TokenType::Arrow)]))]
#[case("-", Ok(vec![L2TokenType::Normal(TokenType::Minus)]))]
#[case("**", Ok(vec![L2TokenType::Normal(TokenType::DoubleAsterisk)]))]
#[case("*", Ok(vec![L2TokenType::Normal(TokenType::Asterisk)]))]
#[case("|>", Ok(vec![L2TokenType::Normal(TokenType::PipeForward)]))]
#[case("|>>", Ok(vec![L2TokenType::Normal(TokenType::PipeDoubleForward)]))]
#[case("|", Ok(vec![L2TokenType::Normal(TokenType::Pipe)]))]
#[case(">=", Ok(vec![L2TokenType::Normal(TokenType::GreaterOrEqual)]))]
#[case(">>", Ok(vec![L2TokenType::Normal(TokenType::DoubleGreater)]))]
#[case(">", Ok(vec![L2TokenType::Normal(TokenType::Greater)]))]
#[case("<=", Ok(vec![L2TokenType::Normal(TokenType::LesserOrEqual)]))]
#[case("<<", Ok(vec![L2TokenType::Normal(TokenType::DoubleLesser)]))]
#[case("<", Ok(vec![L2TokenType::Normal(TokenType::Lesser)]))]
#[case("!hello", Ok(vec![L2TokenType::Normal(TokenType::MacroIdentifier("hello"))]))]
#[case("!", Ok(vec![L2TokenType::Normal(TokenType::Promotion)]))]
#[case("(", Ok(vec![L2TokenType::Brace(Brace::Round, BraceDirection::Open)]))]
#[case(")", Ok(vec![L2TokenType::Brace(Brace::Round, BraceDirection::Close)]))]
#[case("[", Ok(vec![L2TokenType::Brace(Brace::Square, BraceDirection::Open)]))]
#[case("]", Ok(vec![L2TokenType::Brace(Brace::Square, BraceDirection::Close)]))]
#[case("{", Ok(vec![L2TokenType::Brace(Brace::Curly, BraceDirection::Open)]))]
#[case("}", Ok(vec![L2TokenType::Brace(Brace::Curly, BraceDirection::Close)]))]
#[case("\n", Ok(vec![L2TokenType::Newline]))]
#[case("\t", Err(Error::InvalidWhitespace("\t".to_string())))]
#[case("a\nb", Ok(vec![
    L2TokenType::Normal(TokenType::Identifier("a")),
    L2TokenType::Newline,
    L2TokenType::Normal(TokenType::Identifier("b")),
]))]
#[case("a\r\nb", Ok(vec![
    L2TokenType::Normal(TokenType::Identifier("a")),
    L2TokenType::Newline,
    L2TokenType::Normal(TokenType::Identifier("b")),
]))]
#[case("a\tb", Err(Error::InvalidWhitespace("\t".to_string())))]
#[case("fn add(x, y) { return x + y; }", Ok(vec![
    L2TokenType::Normal(TokenType::Function),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Identifier("add")),
    L2TokenType::Brace(Brace::Round, BraceDirection::Open),
    L2TokenType::Normal(TokenType::Identifier("x")),
    L2TokenType::Normal(TokenType::Comma),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Identifier("y")),
    L2TokenType::Brace(Brace::Round, BraceDirection::Close),
    L2TokenType::Whitespace(1),
    L2TokenType::Brace(Brace::Curly, BraceDirection::Open),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Return),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Identifier("x")),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Plus),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Identifier("y")),
    L2TokenType::Normal(TokenType::Semicolon),
    L2TokenType::Whitespace(1),
    L2TokenType::Brace(Brace::Curly, BraceDirection::Close),
]))]
#[case("x = 12.5", Ok(vec![
    L2TokenType::Normal(TokenType::Identifier("x")),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Assignment),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Float("12", Some("5"))),
]))]
#[case("\"hello\"", Ok(vec![
    L2TokenType::Normal(TokenType::String("hello", StringType::Normal)),
]))]
#[case("x = \"value\"", Ok(vec![
    L2TokenType::Normal(TokenType::Identifier("x")),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Assignment),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::String("value", StringType::Normal)),
]))]
#[case("f\"value ${name}\"", Ok(vec![
    L2TokenType::Normal(TokenType::String("value ${name}", StringType::Formatted)),
]))]
#[case("print(f\"value: ${value}\")", Ok(vec![
    L2TokenType::Normal(TokenType::Identifier("print")),
    L2TokenType::Brace(Brace::Round, BraceDirection::Open),
    L2TokenType::Normal(TokenType::String("value: ${value}", StringType::Formatted)),
    L2TokenType::Brace(Brace::Round, BraceDirection::Close),
]))]
#[case("!macrocall", Ok(vec![L2TokenType::Normal(TokenType::MacroIdentifier("macrocall"))]))]
#[case("if true { x } else { y }", Ok(vec![
    L2TokenType::Normal(TokenType::If),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::True),
    L2TokenType::Whitespace(1),
    L2TokenType::Brace(Brace::Curly, BraceDirection::Open),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Identifier("x")),
    L2TokenType::Whitespace(1),
    L2TokenType::Brace(Brace::Curly, BraceDirection::Close),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Else),
    L2TokenType::Whitespace(1),
    L2TokenType::Brace(Brace::Curly, BraceDirection::Open),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Identifier("y")),
    L2TokenType::Whitespace(1),
    L2TokenType::Brace(Brace::Curly, BraceDirection::Close),
]))]
#[case("a .. b", Ok(vec![
    L2TokenType::Normal(TokenType::Identifier("a")),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::DoubleDot),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Identifier("b")),
]))]
#[case("x |> f |>> g", Ok(vec![
    L2TokenType::Normal(TokenType::Identifier("x")),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::PipeForward),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Identifier("f")),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::PipeDoubleForward),
    L2TokenType::Whitespace(1),
    L2TokenType::Normal(TokenType::Identifier("g")),
]))]
#[case("a  b", Ok(vec![
    L2TokenType::Normal(TokenType::Identifier("a")),
    L2TokenType::Whitespace(2),
    L2TokenType::Normal(TokenType::Identifier("b")),
]))]
#[case("x   =   y", Ok(vec![
    L2TokenType::Normal(TokenType::Identifier("x")),
    L2TokenType::Whitespace(3),
    L2TokenType::Normal(TokenType::Assignment),
    L2TokenType::Whitespace(3),
    L2TokenType::Normal(TokenType::Identifier("y")),
]))]
#[case("\r", Err(Error::InvalidWhitespace("\r".to_string())))]
#[case("\"\"", Ok(vec![L2TokenType::Normal(TokenType::String("", StringType::Normal))]))]
#[case("12abc", Ok(vec![
    L2TokenType::Normal(TokenType::Number("12")),
    L2TokenType::Normal(TokenType::Identifier("abc")),
]))]
#[case("\n\n", Ok(vec![L2TokenType::Newline, L2TokenType::Newline]))]
#[case("\x01", Err(Error::UnknownToken("\x01".to_string())))]
#[case("\"", Err(Error::UnterminatedString(0)))]
#[case("x = \"unterminated", Err(Error::UnterminatedString(4)))]
#[case("f\"unterminated ${name}", Err(Error::UnterminatedString(1)))]
fn test_stage2_tokenization(#[case] source: &str, #[case] expected: Res<Vec<L2TokenType>>) {
    assert_eq!(
        l2_tokenize(l1_tokenize(source))
            .map(|token| {
                let token = token?;
                Ok(token.token_type)
            })
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
