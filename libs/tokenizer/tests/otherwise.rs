mod common;

use common::S::{Close, Open, T};
use common::{S, collect};
use tokenizer::{LexType, Res};

#[rstest::rstest]
// Bare otherwise keyword with no body
#[case(
    "otherwise",
    Ok(vec![T(LexType::Otherwise)])
)]
// Basic otherwise with single-statement body
#[case(
    "x := foo\notherwise\n    x := default",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("foo")),
        T(LexType::Otherwise),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("default")),
        Close,
    ])
)]
// Otherwise with multi-statement body
#[case(
    "x := foo\notherwise\n    x := fallback\n    y := 0",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("foo")),
        T(LexType::Otherwise),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("fallback")),
        T(LexType::Identifier("y")),
        T(LexType::StaticAssignment),
        T(LexType::Number("0")),
        Close,
    ])
)]
// Otherwise with return in body
#[case(
    "x := compute\notherwise\n    return default",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("compute")),
        T(LexType::Otherwise),
        Open,
        T(LexType::Return),
        T(LexType::Identifier("default")),
        Close,
    ])
)]
// Otherwise inside function body
#[case(
    "fn f\n    x := foo\n    otherwise\n        x := default\n    return x",
    Ok(vec![
        T(LexType::Function),
        T(LexType::Identifier("f")),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("foo")),
        T(LexType::Otherwise),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("default")),
        Close,
        T(LexType::Return),
        T(LexType::Identifier("x")),
        Close,
    ])
)]
// Chained — statement, otherwise, then more statements
#[case(
    "x := risky\notherwise\n    x := safe\ny := done",
    Ok(vec![
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("risky")),
        T(LexType::Otherwise),
        Open,
        T(LexType::Identifier("x")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("safe")),
        Close,
        T(LexType::Identifier("y")),
        T(LexType::StaticAssignment),
        T(LexType::Identifier("done")),
    ])
)]
fn test_otherwise(#[case] source: &str, #[case] expected: Res<Vec<S<'static>>>) {
    assert_eq!(collect(source), expected);
}
