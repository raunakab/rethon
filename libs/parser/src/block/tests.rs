use lexer::lex;

use super::{BlockTerminator, parse_block, parse_indented_block, parse_optional_indented_block};

#[test]
#[ignore]
fn test_block_colon_terminated() {
    let mut tokens = lex("x:");
    parse_block(&mut tokens, Some(BlockTerminator::Colon)).unwrap();
}

#[test]
#[ignore]
fn test_block_complex_colon_terminated() {
    let mut tokens = lex("x + y:");
    parse_block(&mut tokens, Some(BlockTerminator::Colon)).unwrap();
}

#[test]
#[ignore]
fn test_block_comma_terminated() {
    let mut tokens = lex("x,");
    parse_block(&mut tokens, Some(BlockTerminator::Comma)).unwrap();
}

#[test]
#[ignore]
fn test_block_fat_arrow_terminated() {
    let mut tokens = lex("x =>");
    parse_block(&mut tokens, Some(BlockTerminator::FatArrow)).unwrap();
}

#[test]
#[ignore]
fn test_block_no_terminator() {
    let mut tokens = lex("x");
    parse_block(&mut tokens, None).unwrap();
}

#[test]
#[ignore]
fn test_indented_block_single() {
    let mut tokens = lex("x");
    parse_indented_block(&mut tokens, 0).unwrap();
}

#[test]
#[ignore]
fn test_indented_block_multiple() {
    let mut tokens = lex("x\ny\nz");
    parse_indented_block(&mut tokens, 0).unwrap();
}

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
