use crate::{
    Error, Res,
    l2_tokenizer::{Brace, BraceDirection, L2TokenType, StringType, l2_tokenize},
};

#[rstest::rstest]
#[case("", Ok(vec![]))]
#[case("a", Ok(vec![L2TokenType::Identifier("a")]))]
#[case("ab", Ok(vec![L2TokenType::Identifier("ab")]))]
#[case("fn", Ok(vec![L2TokenType::Function]))]
#[case("return", Ok(vec![L2TokenType::Return]))]
#[case("if", Ok(vec![L2TokenType::If]))]
#[case("else", Ok(vec![L2TokenType::Else]))]
#[case("true", Ok(vec![L2TokenType::True]))]
#[case("false", Ok(vec![L2TokenType::False]))]
#[case("12", Ok(vec![L2TokenType::Number("12")]))]
#[case("12.34", Ok(vec![L2TokenType::Float("12", Some("34"))]))]
#[case("12.", Ok(vec![L2TokenType::Float("12", None)]))]
#[case("0.5", Ok(vec![L2TokenType::Float("0", Some("5"))]))]
#[case("==", Ok(vec![L2TokenType::Equals]))]
#[case("=", Ok(vec![L2TokenType::Assignment]))]
#[case(":=", Ok(vec![L2TokenType::ConstantAssignment]))]
#[case("..", Ok(vec![L2TokenType::DoubleDot]))]
#[case(".", Ok(vec![L2TokenType::Dot]))]
#[case("--", Ok(vec![L2TokenType::DoubleMinus]))]
#[case("->", Ok(vec![L2TokenType::Arrow]))]
#[case("-", Ok(vec![L2TokenType::Minus]))]
#[case("**", Ok(vec![L2TokenType::DoubleAsterisk]))]
#[case("*", Ok(vec![L2TokenType::Asterisk]))]
#[case("|>", Ok(vec![L2TokenType::PipeForward]))]
#[case("|>>", Ok(vec![L2TokenType::PipeDoubleForward]))]
#[case("|", Ok(vec![L2TokenType::Pipe]))]
#[case(">=", Ok(vec![L2TokenType::GreaterOrEqual]))]
#[case(">>", Ok(vec![L2TokenType::DoubleGreater]))]
#[case(">", Ok(vec![L2TokenType::Greater]))]
#[case("<=", Ok(vec![L2TokenType::LesserOrEqual]))]
#[case("<<", Ok(vec![L2TokenType::DoubleLesser]))]
#[case("<", Ok(vec![L2TokenType::Lesser]))]
#[case("!hello", Ok(vec![L2TokenType::MacroIdentifier("hello")]))]
#[case("!", Ok(vec![L2TokenType::Promotion]))]
#[case("(", Ok(vec![L2TokenType::Brace(Brace::Round, BraceDirection::Open)]))]
#[case(")", Ok(vec![L2TokenType::Brace(Brace::Round, BraceDirection::Close)]))]
#[case("[", Ok(vec![L2TokenType::Brace(Brace::Square, BraceDirection::Open)]))]
#[case("]", Ok(vec![L2TokenType::Brace(Brace::Square, BraceDirection::Close)]))]
#[case("{", Ok(vec![L2TokenType::Brace(Brace::Curly, BraceDirection::Open)]))]
#[case("}", Ok(vec![L2TokenType::Brace(Brace::Curly, BraceDirection::Close)]))]
#[case("\n", Ok(vec![L2TokenType::Newline]))]
#[case("\t", Ok(vec![L2TokenType::Tab]))]
#[case("a\nb", Ok(vec![
    L2TokenType::Identifier("a"),
    L2TokenType::Newline,
    L2TokenType::Identifier("b"),
]))]
#[case("a\tb", Ok(vec![
    L2TokenType::Identifier("a"),
    L2TokenType::Tab,
    L2TokenType::Identifier("b"),
]))]
#[case("fn add(x, y) { return x + y; }", Ok(vec![
    L2TokenType::Function,
    L2TokenType::Whitespace(1),
    L2TokenType::Identifier("add"),
    L2TokenType::Brace(Brace::Round, BraceDirection::Open),
    L2TokenType::Identifier("x"),
    L2TokenType::Comma,
    L2TokenType::Whitespace(1),
    L2TokenType::Identifier("y"),
    L2TokenType::Brace(Brace::Round, BraceDirection::Close),
    L2TokenType::Whitespace(1),
    L2TokenType::Brace(Brace::Curly, BraceDirection::Open),
    L2TokenType::Whitespace(1),
    L2TokenType::Return,
    L2TokenType::Whitespace(1),
    L2TokenType::Identifier("x"),
    L2TokenType::Whitespace(1),
    L2TokenType::Plus,
    L2TokenType::Whitespace(1),
    L2TokenType::Identifier("y"),
    L2TokenType::Semicolon,
    L2TokenType::Whitespace(1),
    L2TokenType::Brace(Brace::Curly, BraceDirection::Close),
]))]
#[case("x = 12.5", Ok(vec![
    L2TokenType::Identifier("x"),
    L2TokenType::Whitespace(1),
    L2TokenType::Assignment,
    L2TokenType::Whitespace(1),
    L2TokenType::Float("12", Some("5")),
]))]
#[case("\"hello\"", Ok(vec![
    L2TokenType::String("hello", StringType::Normal),
]))]
#[case("x = \"value\"", Ok(vec![
    L2TokenType::Identifier("x"),
    L2TokenType::Whitespace(1),
    L2TokenType::Assignment,
    L2TokenType::Whitespace(1),
    L2TokenType::String("value", StringType::Normal),
]))]
#[case("f\"value ${name}\"", Ok(vec![
    L2TokenType::String("value ${name}", StringType::Formatted),
]))]
#[case("print(f\"value: ${value}\")", Ok(vec![
    L2TokenType::Identifier("print"),
    L2TokenType::Brace(Brace::Round, BraceDirection::Open),
    L2TokenType::String("value: ${value}", StringType::Formatted),
    L2TokenType::Brace(Brace::Round, BraceDirection::Close),
]))]
#[case("!macrocall", Ok(vec![L2TokenType::MacroIdentifier("macrocall")]))]
#[case("if true { x } else { y }", Ok(vec![
    L2TokenType::If,
    L2TokenType::Whitespace(1),
    L2TokenType::True,
    L2TokenType::Whitespace(1),
    L2TokenType::Brace(Brace::Curly, BraceDirection::Open),
    L2TokenType::Whitespace(1),
    L2TokenType::Identifier("x"),
    L2TokenType::Whitespace(1),
    L2TokenType::Brace(Brace::Curly, BraceDirection::Close),
    L2TokenType::Whitespace(1),
    L2TokenType::Else,
    L2TokenType::Whitespace(1),
    L2TokenType::Brace(Brace::Curly, BraceDirection::Open),
    L2TokenType::Whitespace(1),
    L2TokenType::Identifier("y"),
    L2TokenType::Whitespace(1),
    L2TokenType::Brace(Brace::Curly, BraceDirection::Close),
]))]
#[case("a .. b", Ok(vec![
    L2TokenType::Identifier("a"),
    L2TokenType::Whitespace(1),
    L2TokenType::DoubleDot,
    L2TokenType::Whitespace(1),
    L2TokenType::Identifier("b"),
]))]
#[case("x |> f |>> g", Ok(vec![
    L2TokenType::Identifier("x"),
    L2TokenType::Whitespace(1),
    L2TokenType::PipeForward,
    L2TokenType::Whitespace(1),
    L2TokenType::Identifier("f"),
    L2TokenType::Whitespace(1),
    L2TokenType::PipeDoubleForward,
    L2TokenType::Whitespace(1),
    L2TokenType::Identifier("g"),
]))]
#[case("a  b", Ok(vec![
    L2TokenType::Identifier("a"),
    L2TokenType::Whitespace(2),
    L2TokenType::Identifier("b"),
]))]
#[case("x   =   y", Ok(vec![
    L2TokenType::Identifier("x"),
    L2TokenType::Whitespace(3),
    L2TokenType::Assignment,
    L2TokenType::Whitespace(3),
    L2TokenType::Identifier("y"),
]))]
#[case("\"", Err(Error::UnterminatedString(0)))]
#[case("x = \"unterminated", Err(Error::UnterminatedString(4)))]
#[case("f\"unterminated ${name}", Err(Error::UnterminatedString(1)))]
fn test_tokenization(#[case] source: &str, #[case] expected: Res<Vec<L2TokenType>>) {
    assert_eq!(
        l2_tokenize(source)
            .map(|token| {
                let token = token?;
                Ok(token.token_type)
            })
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
