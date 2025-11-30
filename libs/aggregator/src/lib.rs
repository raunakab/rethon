pub fn aggregator<'a>(source: &'a str) -> impl Iterator<Item = Token<'a>> {
    Aggregator::new(source, tokenizer::tokenize(source))
}

struct Aggregator<'a, I>
where
    I: Iterator<Item = tokenizer::Token<'a>>,
{
    source: &'a str,
    tokenizer: std::iter::Peekable<I>,
}

impl<'a, I> Aggregator<'a, I>
where
    I: Iterator<Item = tokenizer::Token<'a>>,
{
    fn new(source: &'a str, tokenizer: I) -> Self {
        Self {
            source,
            tokenizer: tokenizer.peekable(),
        }
    }
}

impl<'a, I> Iterator for Aggregator<'a, I>
where
    I: Iterator<Item = tokenizer::Token<'a>>,
{
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Helper macro to peek and match on next token
        // Consumes the token if it matches any of the patterns
        // Automatically wraps tuple patterns in Some()
        macro_rules! peek {
            ($(($($inner:tt)*) => $result:expr),+ $(, _ => $default:expr)? $(,)?) => {{
                let next_token = self.tokenizer.peek().map(|t| (t.token, t.range.clone(), t.token_type));
                match next_token {
                    $(
                        Some(($($inner)*)) => {
                            self.tokenizer.next();
                            $result
                        }
                    )+
                    $(
                        _ => $default
                    )?
                }
            }};
        }

        loop {
            let Some(tokenizer::Token {
                token,
                range,
                token_type,
            }) = self.tokenizer.next()
            else {
                break None;
            };

            break Some(match token_type {
                tokenizer::TokenType::Whitespace => match token {
                    "\n" => Token::Newline,
                    "\t" => Token::Tab,
                    _ => continue, // Skip other whitespace (spaces, etc.)
                },
                tokenizer::TokenType::Keyword => match token {
                    "fn" => Token::Function,
                    "scope" => Token::Scope,
                    "return" => Token::Return,
                    "yield" => Token::Yield,
                    "not" => Token::Not,
                    "and" => Token::And,
                    "or" => Token::Or,
                    "for" => Token::For,
                    "loop" => Token::Loop,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "true" => Token::True,
                    "false" => Token::False,
                    _ => Token::Identifier(token),
                },
                tokenizer::TokenType::Numeric => peek! {
                    (".", dot_range, _) => {
                        peek! {
                            (_, frac_range, tokenizer::TokenType::Numeric) => Token::Number(&self.source[range.start..frac_range.end]),
                            _ => Token::Number(&self.source[range.start..dot_range.end]),
                        }
                    },
                    _ => Token::Number(token),
                },
                tokenizer::TokenType::Punctuation => match token {
                    "=" => peek! {
                        ("=", ..) => Token::Equals,
                        _ => Token::Assignment,
                    },
                    "!" => peek! {
                        (ident, _, tokenizer::TokenType::Keyword) => Token::MacroIdentifier(ident),
                        _ => Token::Promotion,
                    },
                    "?" => Token::Coalescence,
                    "@" => Token::Ampersand,
                    ":" => peek! {
                        ("=", ..) => Token::ConstantAssignment,
                        _ => Token::Colon,
                    },
                    "." => peek! {
                        (".", ..) => Token::DoubleDot,
                        _ => Token::Dot,
                    },
                    "+" => Token::Plus,
                    "-" => peek! {
                        ("-", ..) => Token::DoubleMinus,
                        (">", ..) => Token::Arrow,
                        _ => Token::Minus,
                    },
                    "*" => peek! {
                        ("*", ..) => Token::DoubleAsterisk,
                        _ => Token::Asterisk,
                    },
                    "/" => Token::Slash,
                    "|" => peek! {
                        (">", ..) => peek! {
                            (">", ..) => Token::PipeDoubleForward,
                            _ => Token::PipeForward,
                        },
                        _ => Token::Pipe,
                    },
                    ">" => peek! {
                        ("=", ..) => Token::GreaterOrEqual,
                        (">", ..) => Token::DoubleGreater,
                        _ => Token::Greater,
                    },
                    "<" => peek! {
                        ("=", ..) => Token::LesserOrEqual,
                        ("<", ..) => Token::DoubleLesser,
                        _ => Token::Lesser,
                    },
                    "(" => Token::Brace(Brace::Round, BraceDirection::Open),
                    ")" => Token::Brace(Brace::Round, BraceDirection::Close),
                    "[" => Token::Brace(Brace::Square, BraceDirection::Open),
                    "]" => Token::Brace(Brace::Square, BraceDirection::Close),
                    "{" => Token::Brace(Brace::Curly, BraceDirection::Open),
                    "}" => Token::Brace(Brace::Curly, BraceDirection::Close),
                    _ => continue, // Skip unknown punctuation
                },
                tokenizer::TokenType::Unknown => continue,
            });
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    // Control
    Newline,
    Tab,

    // Identifiers
    MacroIdentifier(&'a str),
    Identifier(&'a str),
    Number(&'a str),

    // Keywords
    Function, // fn
    Scope,    // scope
    Return,   // return
    Yield,    // yield
    Not,      // not
    And,      // and
    Or,       // or
    For,      // for
    Loop,     // loop
    If,       // if
    Else,     // else
    True,     // true
    False,    // false

    // Operators
    ConstantAssignment, // :=
    Assignment,         // =
    Equals,             // ==
    Promotion,          // !
    Coalescence,        // ?
    Ampersand,          // @
    Colon,              // :
    Dot,                // .
    DoubleDot,          // ..
    Plus,               // +
    Minus,              // -
    DoubleMinus,        // --
    Arrow,              // ->
    Asterisk,           // *
    DoubleAsterisk,     // **
    Slash,              // /
    Pipe,               // |
    PipeForward,        // |>
    PipeDoubleForward,  // |>>
    Greater,            // >
    DoubleGreater,      // >>
    GreaterOrEqual,     // >=
    Lesser,             // <
    DoubleLesser,       // <<
    LesserOrEqual,      // <=

    // Braces
    Brace(Brace, BraceDirection),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brace {
    Round,
    Square,
    Curly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BraceDirection {
    Open,
    Close,
}
