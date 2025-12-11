use thiserror::Error;

type Res<T = ()> = Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error(transparent)]
    Structurizer(#[from] tokenizer::Error),
}

pub fn parser(source: &str) -> Res<Scope> {
    // let nodes = tokenizer::tokenize(source)?;
    todo!()
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
