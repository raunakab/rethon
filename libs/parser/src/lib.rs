macro_rules! step {
    (
        ($a:pat, $b:pat) = $tokens:expr
        $(,)?
    ) => {
        let result = match $tokens.peek() {
            Some(Ok(..)) => $tokens.next().unwrap().unwrap(),
            Some(Err(..)) => return Err($tokens.next().unwrap().unwrap_err().into()),
            None => return Err(Error::UnexpectedEof),
        };

        let token!($a, $b) = result else { panic!() };
    };
}

macro_rules! stepif {
    (
        ($a:pat, $b:pat) = $tokens:expr
        $(,)?
    ) => {
        let result = match $tokens.peek() {
            Some(Ok(..)) => $tokens.next().unwrap().unwrap(),
            Some(Err(..)) => return Err($tokens.next().unwrap().unwrap_err().into()),
            None => return Ok(None),
        };
        let token!($a, $b) = result else {
            return Ok(None);
        };
    };
}

macro_rules! token {
    (
        $a:pat
        , $b:pat
        $(,)?
    ) => {
        tokenizer::types::Token::Token(
            $a,
            tokenizer::types::Position {
                indentation_level: $b,
                ..
            },
        )
    };
}

use thiserror::Error;
use tokenizer::{tokenize, tokens, types::TokenType};

type Res<T = ()> = Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error(transparent)]
    Tokenizer(#[from] tokenizer::Error),

    #[error("Unexpected EOF encountered")]
    UnexpectedEof,
}

pub fn parser(source: &str) -> Res<Scope> {
    let nodes = tokenize(source);
    todo!()
}

fn parse_scope<'a>(tokens: &mut tokens!('a), indent_level: usize) -> Res<Scope> {
    step!((TokenType::Scope, _) = tokens);
    todo!()
}

fn parse_items<'a>(tokens: &mut tokens!('a), indent_level: usize) -> Res<Scope> {
    todo!()
}

fn parse_item<'a>(tokens: &mut tokens!('a)) -> Res<Item> {
    todo!()
}

fn parse_definition<'a>(tokens: &mut tokens!('a)) -> Res<Definition> {
    let pattern = parse_pattern(tokens)?;
    let r#type = parse_optional_type(tokens)?;
    let value = parse_expression(tokens)?;
    let pattern_match_fail = parse_pattern_match_fail(tokens)?;
    Ok(Definition {
        pattern,
        r#type,
        value,
        pattern_match_fail,
    })
}

fn parse_pattern<'a>(tokens: &mut tokens!('a)) -> Res<Pattern> {
    todo!()
}

fn parse_optional_type<'a>(tokens: &mut tokens!('a)) -> Res<Option<Expression>> {
    stepif!((TokenType::Semicolon, _) = tokens);
    let r#type = parse_expression(tokens)?;
    Ok(Some(r#type))
}

fn parse_expression<'a>(tokens: &mut tokens!('a)) -> Res<Expression> {
    todo!()
}

fn parse_pattern_match_fail<'a>(tokens: &mut tokens!('a)) -> Res<PatternMatchFail> {
    stepif!((TokenType::Otherwise, original_indent_level) = tokens);
    let scope = parse_items(tokens, original_indent_level.checked_add(1).unwrap())?;
    Ok(Some(scope))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Definition(Definition),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub pattern: Pattern,
    pub r#type: Option<Expression>,
    pub value: Expression,
    pub pattern_match_fail: PatternMatchFail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    CatchAll(String),
}

pub type PatternMatchFail = Option<Scope>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    If(Box<If>),
    Function(Box<Function>),
    Loop(Box<Loop>),
    Type(Box<(Expression, Expression)>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If {
    pub condition: Expression,
    pub consequent: Scope,
    pub conditional_antequents: Vec<ElseIf>,
    pub antequent: Option<Scope>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElseIf {
    pub condition: Expression,
    pub consequent: Scope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loop;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum;
