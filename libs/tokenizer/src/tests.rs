use crate::{Token, TokenType, tokenize};

#[rstest::rstest]
#[case("", vec![])]
#[case("a", vec![
    ("a", TokenType::Keyword)
])]
#[case("ab", vec![
    ("ab", TokenType::Keyword)
])]
#[case("a b", vec![
    ("a", TokenType::Keyword),
    (" ", TokenType::Whitespace),
    ("b", TokenType::Keyword),
])]
#[case("a.b", vec![
    ("a", TokenType::Keyword),
    (".", TokenType::Punctuation),
    ("b", TokenType::Keyword),
])]
#[case("0.1", vec![
    ("0", TokenType::Numeric),
    (".", TokenType::Punctuation),
    ("1", TokenType::Numeric),
])]
#[case("!..!", vec![
    ("!", TokenType::Punctuation),
    (".", TokenType::Punctuation),
    (".", TokenType::Punctuation),
    ("!", TokenType::Punctuation),
])]
#[case("Howdy there, partner!!!", vec![
    ("Howdy", TokenType::Keyword),
    (" ", TokenType::Whitespace),
    ("there", TokenType::Keyword),
    (",", TokenType::Punctuation),
    (" ", TokenType::Whitespace),
    ("partner", TokenType::Keyword),
    ("!", TokenType::Punctuation),
    ("!", TokenType::Punctuation),
    ("!", TokenType::Punctuation),
])]
#[case("{ x = 12; y = 1.2; return x + y; }", vec![
    ("{", TokenType::Punctuation),
    (" ", TokenType::Whitespace),
    ("x", TokenType::Keyword),
    (" ", TokenType::Whitespace),
    ("=", TokenType::Punctuation),
    (" ", TokenType::Whitespace),
    ("12", TokenType::Numeric),
    (";", TokenType::Punctuation),
    (" ", TokenType::Whitespace),
    ("y", TokenType::Keyword),
    (" ", TokenType::Whitespace),
    ("=", TokenType::Punctuation),
    (" ", TokenType::Whitespace),
    ("1", TokenType::Numeric),
    (".", TokenType::Punctuation),
    ("2", TokenType::Numeric),
    (";", TokenType::Punctuation),
    (" ", TokenType::Whitespace),
    ("return", TokenType::Keyword),
    (" ", TokenType::Whitespace),
    ("x", TokenType::Keyword),
    (" ", TokenType::Whitespace),
    ("+", TokenType::Punctuation),
    (" ", TokenType::Whitespace),
    ("y", TokenType::Keyword),
    (";", TokenType::Punctuation),
    (" ", TokenType::Whitespace),
    ("}", TokenType::Punctuation),
])]
#[case("🙂🙂🙂", vec![
    ("🙂🙂🙂", TokenType::Keyword),
])]
#[case("🙂🙂🙂 🚀launch🙂🙂!!! 🙃🙂", vec![
    ("🙂🙂🙂", TokenType::Keyword),
    (" ", TokenType::Whitespace),
    ("🚀launch🙂🙂", TokenType::Keyword),
    ("!", TokenType::Punctuation),
    ("!", TokenType::Punctuation),
    ("!", TokenType::Punctuation),
    (" ", TokenType::Whitespace),
    ("🙃🙂", TokenType::Keyword),
])]
#[case("a\nb\nc\nd", vec![
    ("a", TokenType::Keyword),
    ("\n", TokenType::Whitespace),
    ("b", TokenType::Keyword),
    ("\n", TokenType::Whitespace),
    ("c", TokenType::Keyword),
    ("\n", TokenType::Whitespace),
    ("d", TokenType::Keyword),
])]
#[case("a\tb\tc", vec![
    ("a", TokenType::Keyword),
    ("\t", TokenType::Whitespace),
    ("b", TokenType::Keyword),
    ("\t", TokenType::Whitespace),
    ("c", TokenType::Keyword),
])]
#[case("\t\t\tindented", vec![
    ("\t", TokenType::Whitespace),
    ("\t", TokenType::Whitespace),
    ("\t", TokenType::Whitespace),
    ("indented", TokenType::Keyword),
])]
#[case("a\r\nb", vec![
    ("a\r\nb", TokenType::Keyword),
])]
#[case("mixed \t\n whitespace", vec![
    ("mixed", TokenType::Keyword),
    (" ", TokenType::Whitespace),
    ("\t", TokenType::Whitespace),
    ("\n", TokenType::Whitespace),
    (" ", TokenType::Whitespace),
    ("whitespace", TokenType::Keyword),
])]
#[case("tab\tseparated\tvalues", vec![
    ("tab", TokenType::Keyword),
    ("\t", TokenType::Whitespace),
    ("separated", TokenType::Keyword),
    ("\t", TokenType::Whitespace),
    ("values", TokenType::Keyword),
])]
fn test(#[case] source: &str, #[case] expected: Vec<(&str, TokenType)>) {
    assert_eq!(
        tokenize(source)
            .map(
                |Token {
                     token, token_type, ..
                 }| (token, token_type)
            )
            .collect::<Vec<_>>(),
        expected
    );
}
