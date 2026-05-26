#![allow(dead_code, unused_variables, unused_imports)]

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
        lexer::TokenTree::Token(
            $a,
            lexer::Position {
                indentation_level: $b,
                ..
            },
        )
    };
}

mod expressions;
mod literals;

use lexer::{StringType, Token, lex, tokens};
use thiserror::Error;

type Res<T = ()> = Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error(transparent)]
    Tokenizer(#[from] lexer::Error),

    #[error("Unexpected EOF encountered")]
    UnexpectedEof,

    #[error("Ran into an unexpected token, {0}")]
    UnexpectedToken(String),

    #[error("Ran into an unexpected character, {0}")]
    UnexpectedChar(String),
}

pub fn parser<'a>(source: &'a str) -> Res<Block<'a>> {
    let mut tokens = lex(source);
    let block = parse_block(&mut tokens, 0)?;
    Ok(block)
}

fn parse_scope<'a>(tokens: &mut tokens!('a), indent_level: usize) -> Res<Block<'a>> {
    let block = parse_block(tokens, indent_level.checked_add(1).unwrap())?;
    Ok(block)
}

fn parse_block<'a>(tokens: &mut tokens!('a), indent_level: usize) -> Res<Block<'a>> {
    todo!()
}

fn parse_item<'a>(tokens: &mut tokens!('a), indent_level: usize) -> Res<Item<'a>> {
    todo!()
}

fn parse_statement<'a>(tokens: &mut tokens!('a)) -> Res<Statement<'a>> {
    todo!()
    // let pattern = parse_pattern(tokens)?;
    // let r#type = parse_optional_type(tokens)?;
    // let value = parse_expression(tokens)?;
    // let pattern_match_fail = parse_pattern_match_fail(tokens)?;
    // Ok(Statement {
    //     pattern,
    //     r#type,
    //     value,
    //     pattern_match_fail,
    // })
}

fn parse_pattern<'a>(tokens: &mut tokens!('a)) -> Res<Pattern<'a>> {
    step!((Token::Identifier(identifier), _) = tokens);
    Ok(Pattern::Ident(identifier))
}

fn parse_optional_type<'a>(tokens: &mut tokens!('a)) -> Res<Option<Expression<'a>>> {
    stepif!((Token::Semicolon, _) = tokens);
    let r#type = parse_expression(tokens)?;
    Ok(Some(r#type))
}

fn parse_expression<'a>(tokens: &mut tokens!('a)) -> Res<Expression<'a>> {
    Ok(match peek!(tokens) {
        token!(Token::Semicolon, _) => Expression::Noop,
        token!(Token::Comma, _) => {
            return Err(Error::UnexpectedChar(Token::Comma.to_string()));
        }

        token!(Token::MacroIdentifier(_), _) => Expression::Macro(Box::new(parse_macro(tokens)?)),
        _ => todo!(),
    })
}

fn parse_macro<'a>(tokens: &mut tokens!('a)) -> Res<Macro> {
    todo!()
}

fn parse_pattern_match_fail<'a>(_tokens: &mut tokens!('a)) -> Res<Option<Block<'a>>> {
    Ok(None)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern<'a> {
    Ident(&'a str),
    IdentBind(&'a str, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Underscore,

    Literal(Literal<'a>),
    Tuple(Vec<Self>),
    List(Vec<Self>),
    Map(Vec<(Literal<'a>, Self)>),
    Enum(&'a str, Vec<Self>),
    Struct(&'a str, Vec<(&'a str, Self)>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal<'a> {
    True,
    False,
    Number(&'a str),
    Float(&'a str, Option<&'a str>),
    String(&'a str, StringType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<'a> {
    pub items: Vec<Item<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item<'a> {
    Statement(Statement<'a>),
    Expression(Expression<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'a> {
    StaticStatement(StaticStatement<'a>),
    NormalStatement(NormalStatement<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticStatement<'a> {
    pub ident: &'a str,
    pub r#type: Option<Expression<'a>>,
    pub value: Block<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalStatement<'a> {
    pub mutable: bool,
    pub pattern: Pattern<'a>,
    pub r#type: Option<Expression<'a>>,
    pub value: Block<'a>,
    pub r#else: Option<Block<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'a> {
    Noop,

    // Expressions
    Ident(&'a str),
    Literal(Literal<'a>),
    Tuple(Vec<Self>),
    List(Vec<Self>),
    Set(Vec<Self>),
    Map(Vec<(Self, Self)>),

    // Logical Constructs
    If(Box<If<'a>>),
    Match(Box<Match<'a>>),
    Loop(Box<Loop<'a>>),

    // Functions
    Function(Box<Function<'a>>),
    FunctionInvocation(Box<FunctionInvocation<'a>>),
    Return(Option<Block<'a>>),
    Yield(Option<Block<'a>>),
    Throw(Option<Block<'a>>),

    // Typedefs + Type-Invocations
    Struct(Box<Struct<'a>>),
    Enum(Box<Enum<'a>>),

    // Impl-holes
    Panic,
    Todo,
    Unimplemented,

    // Recursion
    Type(Box<(Self, Self)>),
    Block(Block<'a>),

    // Macros
    Macro(Box<Macro>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If<'a> {
    pub condition: Block<'a>,
    pub consequent: Option<Block<'a>>,
    pub conditional_antequents: Vec<ElseIf<'a>>,
    pub antequent: Option<Block<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElseIf<'a> {
    pub condition: Block<'a>,
    pub consequent: Option<Block<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match<'a> {
    pub condition: Block<'a>,
    pub match_arms: Vec<MatchArm<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchArm<'a> {
    pub pattern: Pattern<'a>,
    pub condition: Option<Block<'a>>,
    pub body: Block<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Loop<'a> {
    Unconditional(Option<Block<'a>>),
    Conditional(ConditionalLoop<'a>),
    Iterative(IterativeLoop<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalLoop<'a> {
    pub pattern: Pattern<'a>,
    pub iterative: Block<'a>,
    pub body: Option<Block<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IterativeLoop<'a> {
    pub condition: Block<'a>,
    pub body: Option<Block<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function<'a> {
    pub parameters: Vec<FunctionParameter<'a>>,
    pub return_type: Option<Expression<'a>>,
    pub body: Option<Block<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionParameter<'a> {
    pub pattern: Pattern<'a>,
    pub r#type: Option<Expression<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionInvocation<'a> {
    pub function: Block<'a>,
    pub positional_arguments: Vec<Block<'a>>,
    pub keyword_arguments: Vec<(&'a str, Block<'a>)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct<'a> {
    pub fields: Vec<(&'a str, Expression<'a>)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum<'a> {
    pub discriminants: Vec<(&'a str, Option<EnumVariant<'a>>)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumVariant<'a> {
    Unnamed(Vec<Expression<'a>>),
    Named(Vec<(&'a str, Expression<'a>)>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Macro;
