use crate::{Expression, Item, parser};

#[test]
#[ignore]
fn test_single_arm() {
    let block = parser("match x:\n    y => z").unwrap();
    assert!(matches!(
        block.items.as_slice(),
        [Item::Expression(Expression::Match(_))]
    ));
}

#[test]
#[ignore]
fn test_multiple_arms() {
    let block = parser("match x:\n    a => 1,\n    b => 2").unwrap();
    if let Item::Expression(Expression::Match(m)) = &block.items[0] {
        assert_eq!(m.match_arms.len(), 2);
    } else {
        panic!("expected Match expression");
    }
}

#[test]
#[ignore]
fn test_arm_with_guard() {
    let block = parser("match x:\n    y if cond => z").unwrap();
    if let Item::Expression(Expression::Match(m)) = &block.items[0] {
        assert!(m.match_arms[0].condition.is_some());
    } else {
        panic!("expected Match expression");
    }
}

#[test]
#[ignore]
fn test_arm_without_guard() {
    let block = parser("match x:\n    y => z").unwrap();
    if let Item::Expression(Expression::Match(m)) = &block.items[0] {
        assert!(m.match_arms[0].condition.is_none());
    } else {
        panic!("expected Match expression");
    }
}

#[test]
#[ignore]
fn test_wildcard_arm() {
    let block = parser("match x:\n    _ => z").unwrap();
    assert!(matches!(
        block.items.as_slice(),
        [Item::Expression(Expression::Match(_))]
    ));
}
