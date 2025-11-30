use crate::{Brace, BraceDirection, Res, TokenType, aggregator};

#[rstest::rstest]
#[case("", Ok(vec![]))]
#[case("a", Ok(vec![TokenType::Identifier("a")]))]
#[case("ab", Ok(vec![TokenType::Identifier("ab")]))]
#[case("fn", Ok(vec![TokenType::Function]))]
#[case("return", Ok(vec![TokenType::Return]))]
#[case("if", Ok(vec![TokenType::If]))]
#[case("else", Ok(vec![TokenType::Else]))]
#[case("true", Ok(vec![TokenType::True]))]
#[case("false", Ok(vec![TokenType::False]))]
#[case("12", Ok(vec![TokenType::Number("12")]))]
#[case("12.34", Ok(vec![TokenType::Float("12", Some("34"))]))]
#[case("12.", Ok(vec![TokenType::Float("12", None)]))]
#[case("0.5", Ok(vec![TokenType::Float("0", Some("5"))]))]
#[case("==", Ok(vec![TokenType::Equals]))]
#[case("=", Ok(vec![TokenType::Assignment]))]
#[case(":=", Ok(vec![TokenType::ConstantAssignment]))]
#[case("..", Ok(vec![TokenType::DoubleDot]))]
#[case(".", Ok(vec![TokenType::Dot]))]
#[case("--", Ok(vec![TokenType::DoubleMinus]))]
#[case("->", Ok(vec![TokenType::Arrow]))]
#[case("-", Ok(vec![TokenType::Minus]))]
#[case("**", Ok(vec![TokenType::DoubleAsterisk]))]
#[case("*", Ok(vec![TokenType::Asterisk]))]
#[case("|>", Ok(vec![TokenType::PipeForward]))]
#[case("|>>", Ok(vec![TokenType::PipeDoubleForward]))]
#[case("|", Ok(vec![TokenType::Pipe]))]
#[case(">=", Ok(vec![TokenType::GreaterOrEqual]))]
#[case(">>", Ok(vec![TokenType::DoubleGreater]))]
#[case(">", Ok(vec![TokenType::Greater]))]
#[case("<=", Ok(vec![TokenType::LesserOrEqual]))]
#[case("<<", Ok(vec![TokenType::DoubleLesser]))]
#[case("<", Ok(vec![TokenType::Lesser]))]
#[case("!hello", Ok(vec![TokenType::MacroIdentifier("hello")]))]
#[case("!", Ok(vec![TokenType::Promotion]))]
#[case("(", Ok(vec![TokenType::Brace(Brace::Round, BraceDirection::Open)]))]
#[case(")", Ok(vec![TokenType::Brace(Brace::Round, BraceDirection::Close)]))]
#[case("[", Ok(vec![TokenType::Brace(Brace::Square, BraceDirection::Open)]))]
#[case("]", Ok(vec![TokenType::Brace(Brace::Square, BraceDirection::Close)]))]
#[case("{", Ok(vec![TokenType::Brace(Brace::Curly, BraceDirection::Open)]))]
#[case("}", Ok(vec![TokenType::Brace(Brace::Curly, BraceDirection::Close)]))]
#[case("\n", Ok(vec![TokenType::Newline]))]
#[case("\t", Ok(vec![TokenType::Tab]))]
#[case("a\nb", Ok(vec![
    TokenType::Identifier("a"),
    TokenType::Newline,
    TokenType::Identifier("b"),
]))]
#[case("a\tb", Ok(vec![
    TokenType::Identifier("a"),
    TokenType::Tab,
    TokenType::Identifier("b"),
]))]
#[case("fn add(x, y) { return x + y; }", Ok(vec![
    TokenType::Function,
    TokenType::Whitespace(1),
    TokenType::Identifier("add"),
    TokenType::Brace(Brace::Round, BraceDirection::Open),
    TokenType::Identifier("x"),
    TokenType::Whitespace(1),
    TokenType::Identifier("y"),
    TokenType::Brace(Brace::Round, BraceDirection::Close),
    TokenType::Whitespace(1),
    TokenType::Brace(Brace::Curly, BraceDirection::Open),
    TokenType::Whitespace(1),
    TokenType::Return,
    TokenType::Whitespace(1),
    TokenType::Identifier("x"),
    TokenType::Whitespace(1),
    TokenType::Plus,
    TokenType::Whitespace(1),
    TokenType::Identifier("y"),
    TokenType::Whitespace(1),
    TokenType::Brace(Brace::Curly, BraceDirection::Close),
]))]
#[case("x = 12.5", Ok(vec![
    TokenType::Identifier("x"),
    TokenType::Whitespace(1),
    TokenType::Assignment,
    TokenType::Whitespace(1),
    TokenType::Float("12", Some("5")),
]))]
#[case("!macrocall", Ok(vec![TokenType::MacroIdentifier("macrocall")]))]
#[case("if true { x } else { y }", Ok(vec![
    TokenType::If,
    TokenType::Whitespace(1),
    TokenType::True,
    TokenType::Whitespace(1),
    TokenType::Brace(Brace::Curly, BraceDirection::Open),
    TokenType::Whitespace(1),
    TokenType::Identifier("x"),
    TokenType::Whitespace(1),
    TokenType::Brace(Brace::Curly, BraceDirection::Close),
    TokenType::Whitespace(1),
    TokenType::Else,
    TokenType::Whitespace(1),
    TokenType::Brace(Brace::Curly, BraceDirection::Open),
    TokenType::Whitespace(1),
    TokenType::Identifier("y"),
    TokenType::Whitespace(1),
    TokenType::Brace(Brace::Curly, BraceDirection::Close),
]))]
#[case("a .. b", Ok(vec![
    TokenType::Identifier("a"),
    TokenType::Whitespace(1),
    TokenType::DoubleDot,
    TokenType::Whitespace(1),
    TokenType::Identifier("b"),
]))]
#[case("x |> f |>> g", Ok(vec![
    TokenType::Identifier("x"),
    TokenType::Whitespace(1),
    TokenType::PipeForward,
    TokenType::Whitespace(1),
    TokenType::Identifier("f"),
    TokenType::Whitespace(1),
    TokenType::PipeDoubleForward,
    TokenType::Whitespace(1),
    TokenType::Identifier("g"),
]))]
#[case("a  b", Ok(vec![
    TokenType::Identifier("a"),
    TokenType::Whitespace(2),
    TokenType::Identifier("b"),
]))]
#[case("x   =   y", Ok(vec![
    TokenType::Identifier("x"),
    TokenType::Whitespace(3),
    TokenType::Assignment,
    TokenType::Whitespace(3),
    TokenType::Identifier("y"),
]))]
fn test_valid(#[case] source: &str, #[case] expected: Res<Vec<TokenType>>) {
    assert_eq!(
        aggregator(tokenizer::tokenize(source))
            .map(|token| {
                let token = token?;
                Ok(token.token_type)
            })
            .collect::<Res<Vec<_>>>(),
        expected
    );
}
