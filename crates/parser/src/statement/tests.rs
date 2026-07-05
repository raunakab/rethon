use crate::{Item, Statement, parser};

#[test]
#[ignore]
fn test_static_statement_untyped() {
    let block = parser("x := 1").unwrap();
    if let Item::Statement(Statement::StaticStatement(s)) = &block.items[0] {
        assert!(s.r#type.is_none());
    } else {
        panic!("expected StaticStatement");
    }
}

#[test]
#[ignore]
fn test_static_statement_typed() {
    let block = parser("x: int := 1").unwrap();
    if let Item::Statement(Statement::StaticStatement(s)) = &block.items[0] {
        assert!(s.r#type.is_some());
    } else {
        panic!("expected StaticStatement");
    }
}

#[test]
#[ignore]
fn test_normal_statement_immutable() {
    let block = parser("x = 1").unwrap();
    if let Item::Statement(Statement::NormalStatement(s)) = &block.items[0] {
        assert!(!s.mutable);
        assert!(s.r#else.is_none());
    } else {
        panic!("expected NormalStatement");
    }
}

#[test]
#[ignore]
fn test_normal_statement_mutable() {
    let block = parser("mut x = 1").unwrap();
    if let Item::Statement(Statement::NormalStatement(s)) = &block.items[0] {
        assert!(s.mutable);
    } else {
        panic!("expected NormalStatement");
    }
}

#[test]
#[ignore]
fn test_normal_statement_pattern() {
    let block = parser("(a, b,) = pair").unwrap();
    assert!(matches!(
        block.items.as_slice(),
        [Item::Statement(Statement::NormalStatement(_))]
    ));
}

#[test]
#[ignore]
fn test_normal_statement_typed() {
    let block = parser("x: int = 1").unwrap();
    if let Item::Statement(Statement::NormalStatement(s)) = &block.items[0] {
        assert!(s.r#type.is_some());
    } else {
        panic!("expected NormalStatement");
    }
}

#[test]
#[ignore]
fn test_assignment_with_else() {
    let block = parser("x = f()\nelse:\n    default").unwrap();
    if let Item::Statement(Statement::NormalStatement(s)) = &block.items[0] {
        assert!(s.r#else.is_some());
    } else {
        panic!("expected NormalStatement");
    }
}
