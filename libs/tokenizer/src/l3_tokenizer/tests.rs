use crate::{
    Res,
    l2_tokenizer::L2TokenType,
    l3_tokenizer::{L3Token, l3_tokenize},
};

#[test]
fn test_single_line() {
    let source = "fn add";
    let tokens: Res<Vec<L3Token>> = l3_tokenize(source).collect();
    let tokens = tokens.unwrap();

    assert_eq!(tokens.len(), 3); // "fn", " ", "add"

    // First token: "fn"
    assert_eq!(tokens[0].line, 0);
    assert_eq!(tokens[0].indentation_level, 0);
    assert_eq!(tokens[0].line_range, 0..2);
    assert!(matches!(tokens[0].token_type, L2TokenType::Function));

    // Second token: " "
    assert_eq!(tokens[1].line, 0);
    assert_eq!(tokens[1].indentation_level, 0);

    // Third token: "add"
    assert_eq!(tokens[2].line, 0);
    assert_eq!(tokens[2].indentation_level, 0);
    assert_eq!(tokens[2].line_range, 3..6);
}

#[test]
fn test_multiple_lines() {
    let source = "fn add\nreturn";
    let tokens: Res<Vec<L3Token>> = l3_tokenize(source).collect();
    let tokens = tokens.unwrap();

    // Find the newline and verify line tracking
    let newline_idx = tokens
        .iter()
        .position(|t| matches!(t.token_type, L2TokenType::Newline))
        .unwrap();
    assert_eq!(tokens[newline_idx].line, 0);

    // Token after newline should be on line 1
    let return_idx = tokens
        .iter()
        .position(|t| matches!(t.token_type, L2TokenType::Return))
        .unwrap();
    assert_eq!(tokens[return_idx].line, 1);
    assert_eq!(tokens[return_idx].line_range.start, 0);
}

#[test]
fn test_indentation() {
    let source = "fn\n    add";
    let tokens: Res<Vec<L3Token>> = l3_tokenize(source).collect();
    let tokens = tokens.unwrap();

    // Find the whitespace after newline
    let mut found_indented = false;
    for token in &tokens {
        if token.line == 1 {
            if let L2TokenType::Whitespace(4) = token.token_type {
                assert_eq!(token.indentation_level, 1);
                found_indented = true;
            } else if matches!(token.token_type, L2TokenType::Identifier("add")) {
                // "add" should have indentation_level 1 (inherited from whitespace)
                assert_eq!(token.indentation_level, 1);
            }
        }
    }
    assert!(found_indented);
}

#[test]
fn test_nested_indentation() {
    let source = "fn\n    if\n        x";
    let tokens: Res<Vec<L3Token>> = l3_tokenize(source).collect();
    let tokens = tokens.unwrap();

    // Find tokens on each line
    for token in &tokens {
        match token.line {
            0 => assert_eq!(token.indentation_level, 0),
            1 => {
                if !matches!(token.token_type, L2TokenType::Newline) {
                    assert_eq!(token.indentation_level, 1);
                }
            }
            2 => {
                if !matches!(token.token_type, L2TokenType::Newline) {
                    assert_eq!(token.indentation_level, 2);
                }
            }
            _ => {}
        }
    }
}

#[test]
fn test_invalid_indentation() {
    let source = "fn\n   add"; // 3 spaces, not a multiple of 4
    let tokens: Res<Vec<L3Token>> = l3_tokenize(source).collect();

    assert!(tokens.is_err());
}

#[test]
fn test_line_range_tracking() {
    let source = "abc def\nghi";
    let tokens: Res<Vec<L3Token>> = l3_tokenize(source).collect();
    let tokens = tokens.unwrap();

    // First line: "abc" should be at 0..3, "def" at 4..7
    let abc = tokens
        .iter()
        .find(|t| matches!(t.token_type, L2TokenType::Identifier("abc")))
        .unwrap();
    assert_eq!(abc.line_range, 0..3);

    let def = tokens
        .iter()
        .find(|t| matches!(t.token_type, L2TokenType::Identifier("def")))
        .unwrap();
    assert_eq!(def.line_range, 4..7);

    // Second line: "ghi" should be at 0..3 (line-relative)
    let ghi = tokens
        .iter()
        .find(|t| matches!(t.token_type, L2TokenType::Identifier("ghi")))
        .unwrap();
    assert_eq!(ghi.line, 1);
    assert_eq!(ghi.line_range, 0..3);
}
