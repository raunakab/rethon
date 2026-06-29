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
fn test_wildcard_arm() {
    let block = parser("match x:\n    _ => z").unwrap();
    assert!(matches!(
        block.items.as_slice(),
        [Item::Expression(Expression::Match(_))]
    ));
}

// Structure assertions

fn assert_match_structure(source: &str, arm_count: usize, first_arm_has_guard: bool) {
    let block = parser(source).unwrap();
    if let Item::Expression(Expression::Match(m)) = &block.items[0] {
        assert_eq!(m.match_arms.len(), arm_count);
        assert_eq!(m.match_arms[0].condition.is_some(), first_arm_has_guard);
    } else {
        panic!("expected Match expression");
    }
}

#[test]
#[ignore]
fn test_match_structure_single_arm() {
    assert_match_structure("match x:\n    y => z", 1, false);
}

#[test]
#[ignore]
fn test_match_structure_two_arms() {
    assert_match_structure("match x:\n    a => 1,\n    b => 2", 2, false);
}

#[test]
#[ignore]
fn test_match_structure_with_guard() {
    assert_match_structure("match x:\n    y if cond => z", 1, true);
}

#[test]
#[ignore]
fn test_match_structure_three_arms() {
    assert_match_structure("match x:\n    a => 1,\n    b => 2,\n    c => 3", 3, false);
}
