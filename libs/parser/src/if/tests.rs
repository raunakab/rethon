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
fn test_if_inline_body() {
    // body on the same line as the condition
    let block = parser("if x: y").unwrap();
    assert!(matches!(
        block.items.as_slice(),
        [Item::Expression(Expression::If(_))]
    ));
}

#[test]
#[ignore]
fn test_if_no_else() {
    let block = parser("if x:\n    y").unwrap();
    if let Item::Expression(Expression::If(if_expr)) = &block.items[0] {
        assert!(if_expr.antequent.is_none());
        assert!(if_expr.conditional_antequents.is_empty());
    } else {
        panic!("expected If expression");
    }
}

#[test]
#[ignore]
fn test_else_if_chain_length() {
    let block = parser("if a:\n    1\nelse if b:\n    2\nelse if c:\n    3").unwrap();
    if let Item::Expression(Expression::If(if_expr)) = &block.items[0] {
        assert_eq!(if_expr.conditional_antequents.len(), 2);
        assert!(if_expr.antequent.is_none());
    } else {
        panic!("expected If expression");
    }
}
