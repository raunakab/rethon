use crate::{
    l1_tokenizer::{L1Token, L1TokenType, l1_tokenize},
    types::StringType,
};

#[rstest::rstest]
#[case("", vec![])]
#[case("a", vec![
    ("a", L1TokenType::Keyword)
])]
#[case("ab", vec![
    ("ab", L1TokenType::Keyword)
])]
#[case("a b", vec![
    ("a", L1TokenType::Keyword),
    (" ", L1TokenType::Whitespace),
    ("b", L1TokenType::Keyword),
])]
#[case("a.b", vec![
    ("a", L1TokenType::Keyword),
    (".", L1TokenType::Punctuation),
    ("b", L1TokenType::Keyword),
])]
#[case("0.1", vec![
    ("0", L1TokenType::Numeric),
    (".", L1TokenType::Punctuation),
    ("1", L1TokenType::Numeric),
])]
#[case("!..!", vec![
    ("!", L1TokenType::Punctuation),
    (".", L1TokenType::Punctuation),
    (".", L1TokenType::Punctuation),
    ("!", L1TokenType::Punctuation),
])]
#[case("Howdy there, partner!!!", vec![
    ("Howdy", L1TokenType::Keyword),
    (" ", L1TokenType::Whitespace),
    ("there", L1TokenType::Keyword),
    (",", L1TokenType::Punctuation),
    (" ", L1TokenType::Whitespace),
    ("partner", L1TokenType::Keyword),
    ("!", L1TokenType::Punctuation),
    ("!", L1TokenType::Punctuation),
    ("!", L1TokenType::Punctuation),
])]
#[case("{ x = 12; y = 1.2; return x + y; }", vec![
    ("{", L1TokenType::Punctuation),
    (" ", L1TokenType::Whitespace),
    ("x", L1TokenType::Keyword),
    (" ", L1TokenType::Whitespace),
    ("=", L1TokenType::Punctuation),
    (" ", L1TokenType::Whitespace),
    ("12", L1TokenType::Numeric),
    (";", L1TokenType::Punctuation),
    (" ", L1TokenType::Whitespace),
    ("y", L1TokenType::Keyword),
    (" ", L1TokenType::Whitespace),
    ("=", L1TokenType::Punctuation),
    (" ", L1TokenType::Whitespace),
    ("1", L1TokenType::Numeric),
    (".", L1TokenType::Punctuation),
    ("2", L1TokenType::Numeric),
    (";", L1TokenType::Punctuation),
    (" ", L1TokenType::Whitespace),
    ("return", L1TokenType::Keyword),
    (" ", L1TokenType::Whitespace),
    ("x", L1TokenType::Keyword),
    (" ", L1TokenType::Whitespace),
    ("+", L1TokenType::Punctuation),
    (" ", L1TokenType::Whitespace),
    ("y", L1TokenType::Keyword),
    (";", L1TokenType::Punctuation),
    (" ", L1TokenType::Whitespace),
    ("}", L1TokenType::Punctuation),
])]
#[case("🙂🙂🙂", vec![
    ("🙂🙂🙂", L1TokenType::Keyword),
])]
#[case("🙂🙂🙂 🚀launch🙂🙂!!! 🙃🙂", vec![
    ("🙂🙂🙂", L1TokenType::Keyword),
    (" ", L1TokenType::Whitespace),
    ("🚀launch🙂🙂", L1TokenType::Keyword),
    ("!", L1TokenType::Punctuation),
    ("!", L1TokenType::Punctuation),
    ("!", L1TokenType::Punctuation),
    (" ", L1TokenType::Whitespace),
    ("🙃🙂", L1TokenType::Keyword),
])]
#[case("a\nb\nc\nd", vec![
    ("a", L1TokenType::Keyword),
    ("\n", L1TokenType::Whitespace),
    ("b", L1TokenType::Keyword),
    ("\n", L1TokenType::Whitespace),
    ("c", L1TokenType::Keyword),
    ("\n", L1TokenType::Whitespace),
    ("d", L1TokenType::Keyword),
])]
#[case("a\tb\tc", vec![
    ("a", L1TokenType::Keyword),
    ("\t", L1TokenType::Whitespace),
    ("b", L1TokenType::Keyword),
    ("\t", L1TokenType::Whitespace),
    ("c", L1TokenType::Keyword),
])]
#[case("\t\t\tindented", vec![
    ("\t", L1TokenType::Whitespace),
    ("\t", L1TokenType::Whitespace),
    ("\t", L1TokenType::Whitespace),
    ("indented", L1TokenType::Keyword),
])]
#[case("a\r\nb", vec![
    ("a\r\nb", L1TokenType::Keyword),
])]
#[case("mixed \t\n whitespace", vec![
    ("mixed", L1TokenType::Keyword),
    (" ", L1TokenType::Whitespace),
    ("\t", L1TokenType::Whitespace),
    ("\n", L1TokenType::Whitespace),
    (" ", L1TokenType::Whitespace),
    ("whitespace", L1TokenType::Keyword),
])]
#[case("tab\tseparated\tvalues", vec![
    ("tab", L1TokenType::Keyword),
    ("\t", L1TokenType::Whitespace),
    ("separated", L1TokenType::Keyword),
    ("\t", L1TokenType::Whitespace),
    ("values", L1TokenType::Keyword),
])]
#[case("'", vec![
    ("'", L1TokenType::Punctuation),
])]
#[case("`", vec![
    ("`", L1TokenType::Punctuation),
])]
#[case("\"hello\"", vec![
    ("hello", L1TokenType::String(StringType::Normal)),
])]
#[case("f\"world\"", vec![
    ("world", L1TokenType::String(StringType::Formatted)),
])]
#[case("\"hello\" \"world\"", vec![
    ("hello", L1TokenType::String(StringType::Normal)),
    (" ", L1TokenType::Whitespace),
    ("world", L1TokenType::String(StringType::Normal)),
])]
#[case("f\"formatted\" \"normal\"", vec![
    ("formatted", L1TokenType::String(StringType::Formatted)),
    (" ", L1TokenType::Whitespace),
    ("normal", L1TokenType::String(StringType::Normal)),
])]
#[case("'world'", vec![
    ("'", L1TokenType::Punctuation),
    ("world", L1TokenType::Keyword),
    ("'", L1TokenType::Punctuation),
])]
#[case("`backtick`", vec![
    ("`", L1TokenType::Punctuation),
    ("backtick", L1TokenType::Keyword),
    ("`", L1TokenType::Punctuation),
])]
fn test(#[case] source: &str, #[case] expected: Vec<(&str, L1TokenType)>) {
    assert_eq!(
        l1_tokenize(source)
            .map(|res| {
                let L1Token {
                    token, token_type, ..
                } = res.unwrap();
                (token, token_type)
            })
            .collect::<Vec<_>>(),
        expected
    );
}
