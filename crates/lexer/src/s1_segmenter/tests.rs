use crate::{
    Error, Res,
    s1_segmenter::{Segment, SegmentKind, segment},
};

#[rstest::rstest]
#[case("", vec![])]
#[case("a", vec![("a", SegmentKind::Keyword)])]
#[case("ab", vec![("ab", SegmentKind::Keyword)])]
#[case("a b", vec![
    ("a", SegmentKind::Keyword),
    (" ", SegmentKind::Whitespace),
    ("b", SegmentKind::Keyword),
])]
#[case("a.b", vec![
    ("a", SegmentKind::Keyword),
    (".", SegmentKind::Punctuation),
    ("b", SegmentKind::Keyword),
])]
#[case("0.1", vec![
    ("0", SegmentKind::Numeric),
    (".", SegmentKind::Punctuation),
    ("1", SegmentKind::Numeric),
])]
#[case("!..!", vec![
    ("!", SegmentKind::Punctuation),
    (".", SegmentKind::Punctuation),
    (".", SegmentKind::Punctuation),
    ("!", SegmentKind::Punctuation),
])]
#[case("Howdy there, partner!!!", vec![
    ("Howdy", SegmentKind::Keyword),
    (" ", SegmentKind::Whitespace),
    ("there", SegmentKind::Keyword),
    (",", SegmentKind::Punctuation),
    (" ", SegmentKind::Whitespace),
    ("partner", SegmentKind::Keyword),
    ("!", SegmentKind::Punctuation),
    ("!", SegmentKind::Punctuation),
    ("!", SegmentKind::Punctuation),
])]
#[case("{ x = 12; y = 1.2; return x + y; }", vec![
    ("{", SegmentKind::Punctuation),
    (" ", SegmentKind::Whitespace),
    ("x", SegmentKind::Keyword),
    (" ", SegmentKind::Whitespace),
    ("=", SegmentKind::Punctuation),
    (" ", SegmentKind::Whitespace),
    ("12", SegmentKind::Numeric),
    (";", SegmentKind::Punctuation),
    (" ", SegmentKind::Whitespace),
    ("y", SegmentKind::Keyword),
    (" ", SegmentKind::Whitespace),
    ("=", SegmentKind::Punctuation),
    (" ", SegmentKind::Whitespace),
    ("1", SegmentKind::Numeric),
    (".", SegmentKind::Punctuation),
    ("2", SegmentKind::Numeric),
    (";", SegmentKind::Punctuation),
    (" ", SegmentKind::Whitespace),
    ("return", SegmentKind::Keyword),
    (" ", SegmentKind::Whitespace),
    ("x", SegmentKind::Keyword),
    (" ", SegmentKind::Whitespace),
    ("+", SegmentKind::Punctuation),
    (" ", SegmentKind::Whitespace),
    ("y", SegmentKind::Keyword),
    (";", SegmentKind::Punctuation),
    (" ", SegmentKind::Whitespace),
    ("}", SegmentKind::Punctuation),
])]
#[case("🙂🙂🙂", vec![("🙂🙂🙂", SegmentKind::Keyword)])]
#[case("🙂🙂🙂 🚀launch🙂🙂!!! 🙃🙂", vec![
    ("🙂🙂🙂", SegmentKind::Keyword),
    (" ", SegmentKind::Whitespace),
    ("🚀launch🙂🙂", SegmentKind::Keyword),
    ("!", SegmentKind::Punctuation),
    ("!", SegmentKind::Punctuation),
    ("!", SegmentKind::Punctuation),
    (" ", SegmentKind::Whitespace),
    ("🙃🙂", SegmentKind::Keyword),
])]
#[case("a\nb\nc\nd", vec![
    ("a", SegmentKind::Keyword),
    ("\n", SegmentKind::Whitespace),
    ("b", SegmentKind::Keyword),
    ("\n", SegmentKind::Whitespace),
    ("c", SegmentKind::Keyword),
    ("\n", SegmentKind::Whitespace),
    ("d", SegmentKind::Keyword),
])]
#[case("a\tb\tc", vec![
    ("a", SegmentKind::Keyword),
    ("\t", SegmentKind::Whitespace),
    ("b", SegmentKind::Keyword),
    ("\t", SegmentKind::Whitespace),
    ("c", SegmentKind::Keyword),
])]
#[case("\t\t\tindented", vec![
    ("\t", SegmentKind::Whitespace),
    ("\t", SegmentKind::Whitespace),
    ("\t", SegmentKind::Whitespace),
    ("indented", SegmentKind::Keyword),
])]
#[case("a\r\nb", vec![
    ("a", SegmentKind::Keyword),
    ("\r\n", SegmentKind::Whitespace),
    ("b", SegmentKind::Keyword),
])]
#[case("mixed \t\n whitespace", vec![
    ("mixed", SegmentKind::Keyword),
    (" ", SegmentKind::Whitespace),
    ("\t", SegmentKind::Whitespace),
    ("\n", SegmentKind::Whitespace),
    (" ", SegmentKind::Whitespace),
    ("whitespace", SegmentKind::Keyword),
])]
#[case("tab\tseparated\tvalues", vec![
    ("tab", SegmentKind::Keyword),
    ("\t", SegmentKind::Whitespace),
    ("separated", SegmentKind::Keyword),
    ("\t", SegmentKind::Whitespace),
    ("values", SegmentKind::Keyword),
])]
#[case("'", vec![("'", SegmentKind::Punctuation)])]
#[case("`", vec![("`", SegmentKind::Punctuation)])]
#[case("\"hello\"", vec![("hello", SegmentKind::String)])]
#[case("f\"world\"", vec![
    ("f", SegmentKind::Keyword),
    ("world", SegmentKind::String),
])]
#[case("\"hello\" \"world\"", vec![
    ("hello", SegmentKind::String),
    (" ", SegmentKind::Whitespace),
    ("world", SegmentKind::String),
])]
#[case("f\"formatted\" \"normal\"", vec![
    ("f", SegmentKind::Keyword),
    ("formatted", SegmentKind::String),
    (" ", SegmentKind::Whitespace),
    ("normal", SegmentKind::String),
])]
#[case("'world'", vec![
    ("'", SegmentKind::Punctuation),
    ("world", SegmentKind::Keyword),
    ("'", SegmentKind::Punctuation),
])]
#[case("\"\"", vec![("", SegmentKind::String)])]
#[case("\"hello world\"", vec![("hello world", SegmentKind::String)])]
#[case("\"hi\"there", vec![
    ("hi", SegmentKind::String),
    ("there", SegmentKind::Keyword),
])]
#[case("\r", vec![("\r", SegmentKind::Whitespace)])]
#[case("12abc", vec![
    ("12", SegmentKind::Numeric),
    ("abc", SegmentKind::Keyword),
])]
#[case("\n\n", vec![
    ("\n", SegmentKind::Whitespace),
    ("\n", SegmentKind::Whitespace),
])]
#[case("\x01", vec![("\x01", SegmentKind::Unknown)])]
#[case("`backtick`", vec![
    ("`", SegmentKind::Punctuation),
    ("backtick", SegmentKind::Keyword),
    ("`", SegmentKind::Punctuation),
])]
fn test_stage1_tokenization(#[case] source: &str, #[case] expected: Vec<(&str, SegmentKind)>) {
    assert_eq!(
        segment(source)
            .map(|res| {
                let Segment { segment, kind, .. } = res.unwrap();
                (segment, kind)
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
        segment(source).collect::<Res<Vec<_>>>().unwrap_err(),
        expected
    );
}
