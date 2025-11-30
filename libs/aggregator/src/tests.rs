use crate::{Brace, BraceDirection, Token, TokenType, aggregator};

#[rstest::rstest]
#[case("", vec![])]
#[case("a", vec![TokenType::Identifier("a")])]
#[case("ab", vec![TokenType::Identifier("ab")])]
#[case("fn", vec![TokenType::Function])]
#[case("return", vec![TokenType::Return])]
#[case("if", vec![TokenType::If])]
#[case("else", vec![TokenType::Else])]
#[case("true", vec![TokenType::True])]
#[case("false", vec![TokenType::False])]
#[case("12", vec![TokenType::Number("12")])]
#[case("12.34", vec![TokenType::Float("12", Some("34"))])]
#[case("12.", vec![TokenType::Float("12", None)])]
#[case("0.5", vec![TokenType::Float("0", Some("5"))])]
#[case("==", vec![TokenType::Equals])]
#[case("=", vec![TokenType::Assignment])]
#[case(":=", vec![TokenType::ConstantAssignment])]
#[case("..", vec![TokenType::DoubleDot])]
#[case(".", vec![TokenType::Dot])]
#[case("--", vec![TokenType::DoubleMinus])]
#[case("->", vec![TokenType::Arrow])]
#[case("-", vec![TokenType::Minus])]
#[case("**", vec![TokenType::DoubleAsterisk])]
#[case("*", vec![TokenType::Asterisk])]
#[case("|>", vec![TokenType::PipeForward])]
#[case("|>>", vec![TokenType::PipeDoubleForward])]
#[case("|", vec![TokenType::Pipe])]
#[case(">=", vec![TokenType::GreaterOrEqual])]
#[case(">>", vec![TokenType::DoubleGreater])]
#[case(">", vec![TokenType::Greater])]
#[case("<=", vec![TokenType::LesserOrEqual])]
#[case("<<", vec![TokenType::DoubleLesser])]
#[case("<", vec![TokenType::Lesser])]
#[case("!hello", vec![TokenType::MacroIdentifier("hello")])]
#[case("!", vec![TokenType::Promotion])]
#[case("(", vec![TokenType::Brace(Brace::Round, BraceDirection::Open)])]
#[case(")", vec![TokenType::Brace(Brace::Round, BraceDirection::Close)])]
#[case("[", vec![TokenType::Brace(Brace::Square, BraceDirection::Open)])]
#[case("]", vec![TokenType::Brace(Brace::Square, BraceDirection::Close)])]
#[case("{", vec![TokenType::Brace(Brace::Curly, BraceDirection::Open)])]
#[case("}", vec![TokenType::Brace(Brace::Curly, BraceDirection::Close)])]
#[case("\n", vec![TokenType::Newline])]
#[case("\t", vec![TokenType::Tab])]
#[case("a\nb", vec![
    TokenType::Identifier("a"),
    TokenType::Newline,
    TokenType::Identifier("b"),
])]
#[case("a\tb", vec![
    TokenType::Identifier("a"),
    TokenType::Tab,
    TokenType::Identifier("b"),
])]
#[case("fn add(x, y) { return x + y; }", vec![
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
])]
#[case("x = 12.5", vec![
    TokenType::Identifier("x"),
    TokenType::Whitespace(1),
    TokenType::Assignment,
    TokenType::Whitespace(1),
    TokenType::Float("12", Some("5")),
])]
#[case("!macrocall", vec![TokenType::MacroIdentifier("macrocall")])]
#[case("if true { x } else { y }", vec![
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
])]
#[case("a .. b", vec![
    TokenType::Identifier("a"),
    TokenType::Whitespace(1),
    TokenType::DoubleDot,
    TokenType::Whitespace(1),
    TokenType::Identifier("b"),
])]
#[case("x |> f |>> g", vec![
    TokenType::Identifier("x"),
    TokenType::Whitespace(1),
    TokenType::PipeForward,
    TokenType::Whitespace(1),
    TokenType::Identifier("f"),
    TokenType::Whitespace(1),
    TokenType::PipeDoubleForward,
    TokenType::Whitespace(1),
    TokenType::Identifier("g"),
])]
#[case("a  b", vec![
    TokenType::Identifier("a"),
    TokenType::Whitespace(2),
    TokenType::Identifier("b"),
])]
#[case("x   =   y", vec![
    TokenType::Identifier("x"),
    TokenType::Whitespace(3),
    TokenType::Assignment,
    TokenType::Whitespace(3),
    TokenType::Identifier("y"),
])]
fn test(#[case] source: &str, #[case] expected: Vec<TokenType>) {
    assert_eq!(
        aggregator(tokenizer::tokenize(source))
            .map(|Token { token_type, .. }| token_type)
            .collect::<Vec<_>>(),
        expected
    );
}
