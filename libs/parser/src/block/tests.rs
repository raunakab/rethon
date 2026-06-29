use lexer::lex;

use super::{BlockTerminator, parse_block, parse_indented_block, parse_optional_indented_block};

// --- parse_block ---

#[test]
#[ignore]
fn test_block_single_expr() {
    // `x:` — single identifier, terminated by colon
    let mut tokens = lex("x:");
    parse_block(&mut tokens, Some(BlockTerminator::Colon)).unwrap();
}

#[test]
#[ignore]
fn test_block_complex_expr() {
    let mut tokens = lex("x + y:");
    parse_block(&mut tokens, Some(BlockTerminator::Colon)).unwrap();
}

#[test]
#[ignore]
fn test_block_terminated_by_comma() {
    let mut tokens = lex("x,");
    parse_block(&mut tokens, Some(BlockTerminator::Comma)).unwrap();
}

#[test]
#[ignore]
fn test_block_terminated_by_fat_arrow() {
    let mut tokens = lex("x =>");
    parse_block(&mut tokens, Some(BlockTerminator::FatArrow)).unwrap();
}

#[test]
#[ignore]
fn test_block_no_terminator() {
    let mut tokens = lex("x");
    parse_block(&mut tokens, None).unwrap();
}

// --- parse_indented_block ---

#[test]
#[ignore]
fn test_indented_block_single_item() {
    let mut tokens = lex("x");
    parse_indented_block(&mut tokens, 0).unwrap();
}

#[test]
#[ignore]
fn test_indented_block_multiple_items() {
    let mut tokens = lex("x\ny\nz");
    parse_indented_block(&mut tokens, 0).unwrap();
}

// --- parse_optional_indented_block ---

#[test]
#[ignore]
fn test_optional_indented_block_present() {
    let mut tokens = lex("x\n    y");
    let result = parse_optional_indented_block(&mut tokens, 0).unwrap();
    assert!(result.is_some());
}

#[test]
#[ignore]
fn test_optional_indented_block_absent() {
    let mut tokens = lex("x");
    let result = parse_optional_indented_block(&mut tokens, 0).unwrap();
    assert!(result.is_none());
}
