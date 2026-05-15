use lexer::{Brace, Res, StringType, Token, TokenTree, lex};

#[derive(Debug, PartialEq)]
enum S<'a> {
    T(Token<'a>),
    Open,
    Close,
    BraceOpen(Brace),
    BraceClose(Brace),
}

#[rstest::rstest]
// Empty source
#[case("", Ok(vec![]))]
// Single identifier
#[case("x", Ok(vec![S::T(Token::Identifier("x"))]))]
// Keywords
#[case("fn", Ok(vec![S::T(Token::Function)]))]
#[case("mut", Ok(vec![S::T(Token::Mutable)]))]
#[case("scope", Ok(vec![S::T(Token::Scope)]))]
#[case("return", Ok(vec![S::T(Token::Return)]))]
#[case("yield", Ok(vec![S::T(Token::Yield)]))]
#[case("throw", Ok(vec![S::T(Token::Throw)]))]
#[case("otherwise", Ok(vec![S::T(Token::Otherwise)]))]
#[case("not", Ok(vec![S::T(Token::Not)]))]
#[case("and", Ok(vec![S::T(Token::And)]))]
#[case("or", Ok(vec![S::T(Token::Or)]))]
#[case("for", Ok(vec![S::T(Token::For)]))]
#[case("loop", Ok(vec![S::T(Token::Loop)]))]
#[case("if", Ok(vec![S::T(Token::If)]))]
#[case("else", Ok(vec![S::T(Token::Else)]))]
#[case("match", Ok(vec![S::T(Token::Match)]))]
#[case("struct", Ok(vec![S::T(Token::Struct)]))]
#[case("enum", Ok(vec![S::T(Token::Enum)]))]
#[case("panic", Ok(vec![S::T(Token::Panic)]))]
#[case("todo", Ok(vec![S::T(Token::Todo)]))]
#[case("unimplemented", Ok(vec![S::T(Token::Unimplemented)]))]
#[case("true", Ok(vec![S::T(Token::True)]))]
#[case("false", Ok(vec![S::T(Token::False)]))]
// `f` alone is an identifier, not a formatted string prefix
#[case("f", Ok(vec![S::T(Token::Identifier("f"))]))]
// Numbers
#[case("42", Ok(vec![S::T(Token::Number("42"))]))]
#[case("0", Ok(vec![S::T(Token::Number("0"))]))]
#[case("3.14", Ok(vec![S::T(Token::Float("3", Some("14")))]))]
#[case("1.", Ok(vec![S::T(Token::Float("1", None))]))]
// Strings
#[case("\"hello\"", Ok(vec![S::T(Token::String("hello", StringType::Normal))]))]
#[case("\"\"", Ok(vec![S::T(Token::String("", StringType::Normal))]))]
#[case("f\"hi ${name}\"", Ok(vec![S::T(Token::String("hi ${name}", StringType::Formatted))]))]
// Operators
#[case(":=", Ok(vec![S::T(Token::StaticAssignment)]))]
#[case("=", Ok(vec![S::T(Token::Assignment)]))]
#[case("==", Ok(vec![S::T(Token::Equals)]))]
#[case("!", Ok(vec![S::T(Token::Promotion)]))]
#[case("!foo", Ok(vec![S::T(Token::MacroIdentifier("foo"))]))]
#[case("?", Ok(vec![S::T(Token::Coalescence)]))]
#[case("@", Ok(vec![S::T(Token::Ampersand)]))]
#[case(":", Ok(vec![S::T(Token::Colon)]))]
#[case(".", Ok(vec![S::T(Token::Dot)]))]
#[case("..", Ok(vec![S::T(Token::DoubleDot)]))]
#[case("+", Ok(vec![S::T(Token::Plus)]))]
#[case("-", Ok(vec![S::T(Token::Minus)]))]
#[case("--", Ok(vec![S::T(Token::DoubleMinus)]))]
#[case("->", Ok(vec![S::T(Token::Arrow)]))]
#[case("*", Ok(vec![S::T(Token::Asterisk)]))]
#[case("**", Ok(vec![S::T(Token::DoubleAsterisk)]))]
#[case("/", Ok(vec![S::T(Token::Slash)]))]
#[case(";", Ok(vec![S::T(Token::Semicolon)]))]
#[case(",", Ok(vec![S::T(Token::Comma)]))]
#[case("|", Ok(vec![S::T(Token::Pipe)]))]
#[case("|>", Ok(vec![S::T(Token::PipeForward)]))]
#[case("|>>", Ok(vec![S::T(Token::PipeDoubleForward)]))]
#[case(">", Ok(vec![S::T(Token::Greater)]))]
#[case(">>", Ok(vec![S::T(Token::DoubleGreater)]))]
#[case(">=", Ok(vec![S::T(Token::GreaterOrEqual)]))]
#[case("<", Ok(vec![S::T(Token::Lesser)]))]
#[case("<<", Ok(vec![S::T(Token::DoubleLesser)]))]
#[case("<=", Ok(vec![S::T(Token::LesserOrEqual)]))]
// Braces produce Start/End with brace type
#[case("(", Ok(vec![S::BraceOpen(Brace::Round)]))]
#[case(")", Ok(vec![S::BraceClose(Brace::Round)]))]
#[case("[", Ok(vec![S::BraceOpen(Brace::Square)]))]
#[case("]", Ok(vec![S::BraceClose(Brace::Square)]))]
#[case("{", Ok(vec![S::BraceOpen(Brace::Curly)]))]
#[case("}", Ok(vec![S::BraceClose(Brace::Curly)]))]
// Indentation produces indent scopes
#[case(
    "fn\n    return x",
    Ok(vec![
        S::T(Token::Function),
        S::Open,
        S::T(Token::Return),
        S::T(Token::Identifier("x")),
        S::Close,
    ])
)]
// Function call with parens
#[case(
    "func()",
    Ok(vec![
        S::T(Token::Identifier("func")),
        S::BraceOpen(Brace::Round),
        S::BraceClose(Brace::Round),
    ])
)]
// Multiple tokens on one line
#[case(
    "x := 42",
    Ok(vec![
        S::T(Token::Identifier("x")),
        S::T(Token::StaticAssignment),
        S::T(Token::Number("42")),
    ])
)]
// Newlines between tokens at same level produce no scope change
#[case(
    "a\nb",
    Ok(vec![
        S::T(Token::Identifier("a")),
        S::T(Token::Identifier("b")),
    ])
)]
// Error cases
#[case("\t", Err(lexer::Error::InvalidWhitespace("\t".to_string())))]
#[case("\"unclosed", Err(lexer::Error::UnterminatedString(0)))]
#[case("'", Err(lexer::Error::UnknownItem("'".to_string())))]
#[case("\x01", Err(lexer::Error::UnknownItem("\x01".to_string())))]
#[case("a\n   b", Err(lexer::Error::InvalidIndentation { found: 3, position: 2 }))]
fn test_lex(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(
        lex(source)
            .map(|res| {
                res.map(|node| match node {
                    TokenTree::Token(t, _) => S::T(t),
                    TokenTree::Start(None) => S::Open,
                    TokenTree::End(None) => S::Close,
                    TokenTree::Start(Some((b, _))) => S::BraceOpen(b),
                    TokenTree::End(Some((b, _))) => S::BraceClose(b),
                })
            })
            .collect::<Res<Vec<S<'_>>>>(),
        expected
    );
}
