use crate::{Expression, Item, parser};

#[test]
#[ignore]
fn test_simple_if() {
    let block = parser("if x:\n    y").unwrap();
    assert!(matches!(
        block.items.as_slice(),
        [Item::Expression(Expression::If(_))]
    ));
}

#[test]
#[ignore]
fn test_if_inline_body() {
    let block = parser("if x: y").unwrap();
    assert!(matches!(
        block.items.as_slice(),
        [Item::Expression(Expression::If(_))]
    ));
}

#[test]
#[ignore]
fn test_if_else() {
    let block = parser("if x:\n    y\nelse:\n    z").unwrap();
    assert!(matches!(
        block.items.as_slice(),
        [Item::Expression(Expression::If(_))]
    ));
}

#[test]
#[ignore]
fn test_if_else_if_else() {
    let block = parser("if x:\n    a\nelse if y:\n    b\nelse:\n    c").unwrap();
    assert!(matches!(
        block.items.as_slice(),
        [Item::Expression(Expression::If(_))]
    ));
}

#[test]
#[ignore]
fn test_if_compound_condition() {
    let block = parser("if x and y:\n    z").unwrap();
    assert!(matches!(
        block.items.as_slice(),
        [Item::Expression(Expression::If(_))]
    ));
}

// Structure assertions

fn assert_if_structure(source: &str, else_if_count: usize, has_else: bool) {
    let block = parser(source).unwrap();
    if let Item::Expression(Expression::If(if_expr)) = &block.items[0] {
        assert_eq!(if_expr.conditional_antequents.len(), else_if_count);
        assert_eq!(if_expr.antequent.is_some(), has_else);
    } else {
        panic!("expected If expression");
    }
}

#[test]
#[ignore]
fn test_if_structure_plain() {
    assert_if_structure("if x:\n    y", 0, false);
}

#[test]
#[ignore]
fn test_if_structure_one_else_if() {
    assert_if_structure("if a:\n    1\nelse if b:\n    2", 1, false);
}

#[test]
#[ignore]
fn test_if_structure_one_else_if_and_else() {
    assert_if_structure("if a:\n    1\nelse if b:\n    2\nelse:\n    3", 1, true);
}

#[test]
#[ignore]
fn test_if_structure_two_else_ifs_and_else() {
    assert_if_structure(
        "if a:\n    1\nelse if b:\n    2\nelse if c:\n    3\nelse:\n    4",
        2,
        true,
    );
}
