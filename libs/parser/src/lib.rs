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
        match $tokens.peek() {
            #[allow(unused_variables)]
            Some(Ok(token!($a, $b))) => {}
            Some(Err(..)) => return Err($tokens.next().unwrap().unwrap_err().into()),
            _ => return Ok(None),
        }
        let token!($a, $b) = $tokens.next().unwrap().unwrap() else {
            unreachable!()
        };
    };
}

macro_rules! peek {
    (
        $tokens:expr
        $(,)?
    ) => {
        match $tokens.peek() {
            Some(Ok(token)) => token,
            Some(Err(..)) => return Err($tokens.next().unwrap().unwrap_err().into()),
            None => return Err(Error::UnexpectedEof),
        }
    };
}

macro_rules! token {
    (
        $a:pat
        , $b:pat
        $(,)?
    ) => {
        tokenizer::Token::Token(
            $a,
            tokenizer::Position {
                indentation_level: $b,
                ..
            },
        )
    };
}

use thiserror::Error;
use tokenizer::{TokenType, tokenize, tokens};

type Res<T = ()> = Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error(transparent)]
    Tokenizer(#[from] tokenizer::Error),

    #[error("Unexpected EOF encountered")]
    UnexpectedEof,

    #[error("Ran into an unexpected character, {0}")]
    InvalidChar(String),
}

pub fn parser<'a>(source: &'a str) -> Res<Scope<'a>> {
    let mut tokens = tokenize(source);
    let items = parse_items(&mut tokens, 0)?;
    Ok(items)
}

fn parse_scope<'a>(tokens: &mut tokens!('a), indent_level: usize) -> Res<Scope<'a>> {
    step!((TokenType::Scope, _) = tokens);
    let items = parse_items(tokens, indent_level.checked_add(1).unwrap())?;
    Ok(items)
}

fn parse_items<'a>(tokens: &mut tokens!('a), indent_level: usize) -> Res<Scope<'a>> {
    todo!()
}

fn parse_item<'a>(tokens: &mut tokens!('a), indent_level: usize) -> Res<Item<'a>> {
    todo!()
}

fn parse_definition<'a>(tokens: &mut tokens!('a)) -> Res<Definition<'a>> {
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

fn parse_pattern<'a>(tokens: &mut tokens!('a)) -> Res<Pattern<'a>> {
    step!((TokenType::Identifier(identifier), _) = tokens);
    Ok(Pattern::CatchAll(identifier))
}

fn parse_optional_type<'a>(tokens: &mut tokens!('a)) -> Res<Option<Expression<'a>>> {
    stepif!((TokenType::Semicolon, _) = tokens);
    let r#type = parse_expression(tokens)?;
    Ok(Some(r#type))
}

fn parse_expression<'a>(tokens: &mut tokens!('a)) -> Res<Expression<'a>> {
    Ok(match peek!(tokens) {
        token!(TokenType::Semicolon, _) => Expression::Noop,
        token!(TokenType::Comma, _) => {
            return Err(Error::InvalidChar(TokenType::Comma.to_string()));
        }

        token!(TokenType::MacroIdentifier(_), _) => {
            Expression::Macro(Box::new(parse_macro(tokens)?))
        }
        _ => todo!(),
    })
}

fn parse_macro<'a>(tokens: &mut tokens!('a)) -> Res<Macro> {
    todo!()
}

fn parse_pattern_match_fail<'a>(tokens: &mut tokens!('a)) -> Res<PatternMatchFail<'a>> {
    stepif!((TokenType::Otherwise, original_indent_level) = tokens);
    let scope = parse_items(tokens, original_indent_level.checked_add(1).unwrap())?;
    Ok(Some(scope))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope<'a> {
    pub items: Vec<Item<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item<'a> {
    Definition(Definition<'a>),
    Expression(Expression<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition<'a> {
    pub pattern: Pattern<'a>,
    pub r#type: Option<Expression<'a>>,
    pub value: Expression<'a>,
    pub pattern_match_fail: PatternMatchFail<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern<'a> {
    CatchAll(&'a str),
}

pub type PatternMatchFail<'a> = Option<Scope<'a>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'a> {
    Noop,
    If(Box<If<'a>>),
    Function(Box<Function>),
    Loop(Box<Loop>),
    Type(Box<(Expression<'a>, Expression<'a>)>),
    Macro(Box<Macro>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If<'a> {
    pub condition: Expression<'a>,
    pub consequent: Scope<'a>,
    pub conditional_antequents: Vec<ElseIf<'a>>,
    pub antequent: Option<Scope<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElseIf<'a> {
    pub condition: Expression<'a>,
    pub consequent: Scope<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loop;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Macro;
