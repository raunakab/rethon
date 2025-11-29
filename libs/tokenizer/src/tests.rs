use crate::{GraphemeState, tokenize};

#[rstest::rstest]
#[case("", vec![])]
#[case("a", vec![
    ("a", GraphemeState::Keyword)
])]
#[case("ab", vec![
    ("ab", GraphemeState::Keyword)
])]
#[case("a b", vec![
    ("a", GraphemeState::Keyword),
    (" ", GraphemeState::Whitespace),
    ("b", GraphemeState::Keyword),
])]
#[case("a.b", vec![
    ("a", GraphemeState::Keyword),
    (".", GraphemeState::Punctuation),
    ("b", GraphemeState::Keyword),
])]
#[case("0.1", vec![
    ("0", GraphemeState::Numeric),
    (".", GraphemeState::Punctuation),
    ("1", GraphemeState::Numeric),
])]
#[case("!..!", vec![
    ("!", GraphemeState::Punctuation),
    (".", GraphemeState::Punctuation),
    (".", GraphemeState::Punctuation),
    ("!", GraphemeState::Punctuation),
])]
#[case("Howdy there, partner!!!", vec![
    ("Howdy", GraphemeState::Keyword),
    (" ", GraphemeState::Whitespace),
    ("there", GraphemeState::Keyword),
    (",", GraphemeState::Punctuation),
    (" ", GraphemeState::Whitespace),
    ("partner", GraphemeState::Keyword),
    ("!", GraphemeState::Punctuation),
    ("!", GraphemeState::Punctuation),
    ("!", GraphemeState::Punctuation),
])]
#[case("{ x = 12; y = 1.2; return x + y; }", vec![
    ("{", GraphemeState::Punctuation),
    (" ", GraphemeState::Whitespace),
    ("x", GraphemeState::Keyword),
    (" ", GraphemeState::Whitespace),
    ("=", GraphemeState::Punctuation),
    (" ", GraphemeState::Whitespace),
    ("12", GraphemeState::Numeric),
    (";", GraphemeState::Punctuation),
    (" ", GraphemeState::Whitespace),
    ("y", GraphemeState::Keyword),
    (" ", GraphemeState::Whitespace),
    ("=", GraphemeState::Punctuation),
    (" ", GraphemeState::Whitespace),
    ("1", GraphemeState::Numeric),
    (".", GraphemeState::Punctuation),
    ("2", GraphemeState::Numeric),
    (";", GraphemeState::Punctuation),
    (" ", GraphemeState::Whitespace),
    ("return", GraphemeState::Keyword),
    (" ", GraphemeState::Whitespace),
    ("x", GraphemeState::Keyword),
    (" ", GraphemeState::Whitespace),
    ("+", GraphemeState::Punctuation),
    (" ", GraphemeState::Whitespace),
    ("y", GraphemeState::Keyword),
    (";", GraphemeState::Punctuation),
    (" ", GraphemeState::Whitespace),
    ("}", GraphemeState::Punctuation),
])]
fn test(#[case] source: &str, #[case] expected: Vec<(&str, GraphemeState)>) {
    assert_eq!(tokenize(source).collect::<Vec<_>>(), expected);
}
