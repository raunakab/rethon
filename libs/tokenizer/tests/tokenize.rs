use tokenizer::{Error, Res, StringType, Token, TokenType, tokenize};

// Strips position metadata for readable assertions.
#[derive(Debug, PartialEq)]
enum S<'a> {
    T(TokenType<'a>),
    Open,
    Close,
}

use S::{Close, Open, T};

fn collect(source: &str) -> Res<Vec<S<'_>>> {
    tokenize(source)
        .map(|res| {
            res.map(|token| match token {
                Token::Token(ty, _) => T(ty),
                Token::ScopeStart(_) => Open,
                Token::ScopeEnd(_) => Close,
            })
        })
        .collect()
}

// ── Happy-path tests ─────────────────────────────────────────────────────────

#[test]
fn empty() {
    assert_eq!(collect(""), Ok(vec![]));
}

#[test]
fn constant_assignment() {
    assert_eq!(
        collect("x := 42"),
        Ok(vec![
            T(TokenType::Identifier("x")),
            T(TokenType::ConstantAssignment),
            T(TokenType::Number("42")),
        ])
    );
}

#[test]
fn mutable_assignment() {
    assert_eq!(
        collect("x = 3.14"),
        Ok(vec![
            T(TokenType::Identifier("x")),
            T(TokenType::Assignment),
            T(TokenType::Float("3", Some("14"))),
        ])
    );
}

#[test]
fn function_body() {
    let src = "fn add\n    x := 1\n    return x";
    assert_eq!(
        collect(src),
        Ok(vec![
            T(TokenType::Function),
            T(TokenType::Identifier("add")),
            Open,
            T(TokenType::Identifier("x")),
            T(TokenType::ConstantAssignment),
            T(TokenType::Number("1")),
            T(TokenType::Return),
            T(TokenType::Identifier("x")),
            Close,
        ])
    );
}

#[test]
fn if_else() {
    let src = "if cond\n    return true\nelse\n    return false";
    assert_eq!(
        collect(src),
        Ok(vec![
            T(TokenType::If),
            T(TokenType::Identifier("cond")),
            Open,
            T(TokenType::Return),
            T(TokenType::True),
            Close,
            T(TokenType::Else),
            Open,
            T(TokenType::Return),
            T(TokenType::False),
            Close,
        ])
    );
}

#[test]
fn nested_scopes() {
    let src = "fn check\n    if x\n        return true";
    assert_eq!(
        collect(src),
        Ok(vec![
            T(TokenType::Function),
            T(TokenType::Identifier("check")),
            Open,
            T(TokenType::If),
            T(TokenType::Identifier("x")),
            Open,
            T(TokenType::Return),
            T(TokenType::True),
            Close,
            Close,
        ])
    );
}

#[test]
fn pipe_operators() {
    assert_eq!(
        collect("x |> f |>> g"),
        Ok(vec![
            T(TokenType::Identifier("x")),
            T(TokenType::PipeForward),
            T(TokenType::Identifier("f")),
            T(TokenType::PipeDoubleForward),
            T(TokenType::Identifier("g")),
        ])
    );
}

#[test]
fn boolean_and_comparison_operators() {
    assert_eq!(
        collect("not x and y >= 0 or z == w"),
        Ok(vec![
            T(TokenType::Not),
            T(TokenType::Identifier("x")),
            T(TokenType::And),
            T(TokenType::Identifier("y")),
            T(TokenType::GreaterOrEqual),
            T(TokenType::Number("0")),
            T(TokenType::Or),
            T(TokenType::Identifier("z")),
            T(TokenType::Equals),
            T(TokenType::Identifier("w")),
        ])
    );
}

#[test]
fn arithmetic_operators() {
    assert_eq!(
        collect("a + b - c * d / e ** f"),
        Ok(vec![
            T(TokenType::Identifier("a")),
            T(TokenType::Plus),
            T(TokenType::Identifier("b")),
            T(TokenType::Minus),
            T(TokenType::Identifier("c")),
            T(TokenType::Asterisk),
            T(TokenType::Identifier("d")),
            T(TokenType::Slash),
            T(TokenType::Identifier("e")),
            T(TokenType::DoubleAsterisk),
            T(TokenType::Identifier("f")),
        ])
    );
}

#[test]
fn string_literals() {
    // Normal string
    assert_eq!(
        collect("x := \"hello\""),
        Ok(vec![
            T(TokenType::Identifier("x")),
            T(TokenType::ConstantAssignment),
            T(TokenType::String("hello", StringType::Normal)),
        ])
    );

    // Formatted string
    assert_eq!(
        collect("y := f\"world\""),
        Ok(vec![
            T(TokenType::Identifier("y")),
            T(TokenType::ConstantAssignment),
            T(TokenType::String("world", StringType::Formatted)),
        ])
    );

    // Empty string
    assert_eq!(
        collect("z := \"\""),
        Ok(vec![
            T(TokenType::Identifier("z")),
            T(TokenType::ConstantAssignment),
            T(TokenType::String("", StringType::Normal)),
        ])
    );
}

#[test]
fn macro_invocation() {
    assert_eq!(
        collect("!print x"),
        Ok(vec![
            T(TokenType::MacroIdentifier("print")),
            T(TokenType::Identifier("x")),
        ])
    );
}

#[test]
fn misc_operators() {
    // Coalescence (?), promotion (!), reference (@), range (..), arrow (->)
    assert_eq!(
        collect("x? + @y + z! + a..b + c -> d"),
        Ok(vec![
            T(TokenType::Identifier("x")),
            T(TokenType::Coalescence),
            T(TokenType::Plus),
            T(TokenType::Ampersand),
            T(TokenType::Identifier("y")),
            T(TokenType::Plus),
            T(TokenType::Identifier("z")),
            T(TokenType::Promotion),
            T(TokenType::Plus),
            T(TokenType::Identifier("a")),
            T(TokenType::DoubleDot),
            T(TokenType::Identifier("b")),
            T(TokenType::Plus),
            T(TokenType::Identifier("c")),
            T(TokenType::Arrow),
            T(TokenType::Identifier("d")),
        ])
    );
}

#[test]
fn type_annotation() {
    // Semicolon separates the type from the binding
    assert_eq!(
        collect("x ; Int := 42"),
        Ok(vec![
            T(TokenType::Identifier("x")),
            T(TokenType::Semicolon),
            T(TokenType::Identifier("Int")),
            T(TokenType::ConstantAssignment),
            T(TokenType::Number("42")),
        ])
    );
}

#[test]
fn keywords_control_and_type() {
    // Verify each remaining keyword tokenizes correctly end-to-end
    let src = "mut x = 1\nscope\nfor x\nloop\nyield x\nthrow x\nstruct Foo\nenum Bar\npanic\ntodo\nunimplemented";
    assert_eq!(
        collect(src),
        Ok(vec![
            T(TokenType::Mutable),
            T(TokenType::Identifier("x")),
            T(TokenType::Assignment),
            T(TokenType::Number("1")),
            T(TokenType::Scope),
            T(TokenType::For),
            T(TokenType::Identifier("x")),
            T(TokenType::Loop),
            T(TokenType::Yield),
            T(TokenType::Identifier("x")),
            T(TokenType::Throw),
            T(TokenType::Identifier("x")),
            T(TokenType::Struct),
            T(TokenType::Identifier("Foo")),
            T(TokenType::Enum),
            T(TokenType::Identifier("Bar")),
            T(TokenType::Panic),
            T(TokenType::Todo),
            T(TokenType::Unimplemented),
        ])
    );
}

#[test]
fn otherwise_clause() {
    let src = "x := foo\notherwise\n    x := default";
    assert_eq!(
        collect(src),
        Ok(vec![
            T(TokenType::Identifier("x")),
            T(TokenType::ConstantAssignment),
            T(TokenType::Identifier("foo")),
            T(TokenType::Otherwise),
            Open,
            T(TokenType::Identifier("x")),
            T(TokenType::ConstantAssignment),
            T(TokenType::Identifier("default")),
            Close,
        ])
    );
}

#[test]
fn crlf_line_endings() {
    // CRLF must produce the same token stream as LF
    let lf = collect("fn check\n    return true");
    let crlf = collect("fn check\r\n    return true");
    assert_eq!(lf, crlf);
}

// ── Error tests ───────────────────────────────────────────────────────────────

#[test]
fn error_unterminated_string() {
    // "x := " is 5 bytes, so the opening quote is at byte 5
    assert_eq!(
        collect("x := \"unterminated"),
        Err(Error::UnterminatedString(5))
    );
}

#[test]
fn error_invalid_indentation() {
    // 3 spaces is not a multiple of 4
    assert_eq!(
        collect("fn\n   x"),
        Err(Error::InvalidIndentation {
            found: 3,
            position: 3
        })
    );
}

#[test]
fn error_unexpected_brace() {
    assert_eq!(collect("fn\n    {x}"), Err(Error::UnexpectedBrace));
}
