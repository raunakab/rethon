use crate::{
    Error, Res,
    lexer_s1::{Token, TokenKind, tokenize},
};

#[rstest::rstest]
#[case("", vec![])]
#[case("a", vec![("a", TokenKind::Keyword)])]
#[case("ab", vec![("ab", TokenKind::Keyword)])]
#[case("a b", vec![
    ("a", TokenKind::Keyword),
    (" ", TokenKind::Whitespace),
    ("b", TokenKind::Keyword),
])]
#[case("a.b", vec![
    ("a", TokenKind::Keyword),
    (".", TokenKind::Punctuation),
    ("b", TokenKind::Keyword),
])]
#[case("0.1", vec![
    ("0", TokenKind::Numeric),
    (".", TokenKind::Punctuation),
    ("1", TokenKind::Numeric),
])]
#[case("!..!", vec![
    ("!", TokenKind::Punctuation),
    (".", TokenKind::Punctuation),
    (".", TokenKind::Punctuation),
    ("!", TokenKind::Punctuation),
])]
#[case("Howdy there, partner!!!", vec![
    ("Howdy", TokenKind::Keyword),
    (" ", TokenKind::Whitespace),
    ("there", TokenKind::Keyword),
    (",", TokenKind::Punctuation),
    (" ", TokenKind::Whitespace),
    ("partner", TokenKind::Keyword),
    ("!", TokenKind::Punctuation),
    ("!", TokenKind::Punctuation),
    ("!", TokenKind::Punctuation),
])]
#[case("{ x = 12; y = 1.2; return x + y; }", vec![
    ("{", TokenKind::Punctuation),
    (" ", TokenKind::Whitespace),
    ("x", TokenKind::Keyword),
    (" ", TokenKind::Whitespace),
    ("=", TokenKind::Punctuation),
    (" ", TokenKind::Whitespace),
    ("12", TokenKind::Numeric),
    (";", TokenKind::Punctuation),
    (" ", TokenKind::Whitespace),
    ("y", TokenKind::Keyword),
    (" ", TokenKind::Whitespace),
    ("=", TokenKind::Punctuation),
    (" ", TokenKind::Whitespace),
    ("1", TokenKind::Numeric),
    (".", TokenKind::Punctuation),
    ("2", TokenKind::Numeric),
    (";", TokenKind::Punctuation),
    (" ", TokenKind::Whitespace),
    ("return", TokenKind::Keyword),
    (" ", TokenKind::Whitespace),
    ("x", TokenKind::Keyword),
    (" ", TokenKind::Whitespace),
    ("+", TokenKind::Punctuation),
    (" ", TokenKind::Whitespace),
    ("y", TokenKind::Keyword),
    (";", TokenKind::Punctuation),
    (" ", TokenKind::Whitespace),
    ("}", TokenKind::Punctuation),
])]
#[case("🙂🙂🙂", vec![("🙂🙂🙂", TokenKind::Keyword)])]
#[case("🙂🙂🙂 🚀launch🙂🙂!!! 🙃🙂", vec![
    ("🙂🙂🙂", TokenKind::Keyword),
    (" ", TokenKind::Whitespace),
    ("🚀launch🙂🙂", TokenKind::Keyword),
    ("!", TokenKind::Punctuation),
    ("!", TokenKind::Punctuation),
    ("!", TokenKind::Punctuation),
    (" ", TokenKind::Whitespace),
    ("🙃🙂", TokenKind::Keyword),
])]
#[case("a\nb\nc\nd", vec![
    ("a", TokenKind::Keyword),
    ("\n", TokenKind::Whitespace),
    ("b", TokenKind::Keyword),
    ("\n", TokenKind::Whitespace),
    ("c", TokenKind::Keyword),
    ("\n", TokenKind::Whitespace),
    ("d", TokenKind::Keyword),
])]
#[case("a\tb\tc", vec![
    ("a", TokenKind::Keyword),
    ("\t", TokenKind::Whitespace),
    ("b", TokenKind::Keyword),
    ("\t", TokenKind::Whitespace),
    ("c", TokenKind::Keyword),
])]
#[case("\t\t\tindented", vec![
    ("\t", TokenKind::Whitespace),
    ("\t", TokenKind::Whitespace),
    ("\t", TokenKind::Whitespace),
    ("indented", TokenKind::Keyword),
])]
#[case("a\r\nb", vec![
    ("a", TokenKind::Keyword),
    ("\r\n", TokenKind::Whitespace),
    ("b", TokenKind::Keyword),
])]
#[case("mixed \t\n whitespace", vec![
    ("mixed", TokenKind::Keyword),
    (" ", TokenKind::Whitespace),
    ("\t", TokenKind::Whitespace),
    ("\n", TokenKind::Whitespace),
    (" ", TokenKind::Whitespace),
    ("whitespace", TokenKind::Keyword),
])]
#[case("tab\tseparated\tvalues", vec![
    ("tab", TokenKind::Keyword),
    ("\t", TokenKind::Whitespace),
    ("separated", TokenKind::Keyword),
    ("\t", TokenKind::Whitespace),
    ("values", TokenKind::Keyword),
])]
#[case("'", vec![("'", TokenKind::Punctuation)])]
#[case("`", vec![("`", TokenKind::Punctuation)])]
#[case("\"hello\"", vec![("hello", TokenKind::String)])]
#[case("f\"world\"", vec![
    ("f", TokenKind::Keyword),
    ("world", TokenKind::String),
])]
#[case("\"hello\" \"world\"", vec![
    ("hello", TokenKind::String),
    (" ", TokenKind::Whitespace),
    ("world", TokenKind::String),
])]
#[case("f\"formatted\" \"normal\"", vec![
    ("f", TokenKind::Keyword),
    ("formatted", TokenKind::String),
    (" ", TokenKind::Whitespace),
    ("normal", TokenKind::String),
])]
#[case("'world'", vec![
    ("'", TokenKind::Punctuation),
    ("world", TokenKind::Keyword),
    ("'", TokenKind::Punctuation),
])]
#[case("\"\"", vec![("", TokenKind::String)])]
#[case("\"hello world\"", vec![("hello world", TokenKind::String)])]
#[case("\"hi\"there", vec![
    ("hi", TokenKind::String),
    ("there", TokenKind::Keyword),
])]
#[case("\r", vec![("\r", TokenKind::Whitespace)])]
#[case("12abc", vec![
    ("12", TokenKind::Numeric),
    ("abc", TokenKind::Keyword),
])]
#[case("\n\n", vec![
    ("\n", TokenKind::Whitespace),
    ("\n", TokenKind::Whitespace),
])]
#[case("\x01", vec![("\x01", TokenKind::Unknown)])]
#[case("`backtick`", vec![
    ("`", TokenKind::Punctuation),
    ("backtick", TokenKind::Keyword),
    ("`", TokenKind::Punctuation),
])]
fn test_stage1_tokenization(#[case] source: &str, #[case] expected: Vec<(&str, TokenKind)>) {
    assert_eq!(
        tokenize(source)
            .map(|res| {
                let Token { token, kind, .. } = res.unwrap();
                (token, kind)
            })
            .collect::<Vec<_>>(),
        expected
    );
}

#[rstest::rstest]
#[case("\"", Error::UnterminatedString(0))]
#[case("\"hello", Error::UnterminatedString(0))]
#[case("x \"hello", Error::UnterminatedString(2))]
fn test_stage1_errors(#[case] source: &str, #[case] expected: Error) {
    assert_eq!(
        tokenize(source).collect::<Res<Vec<_>>>().unwrap_err(),
        expected
    );
}
