pub fn aggregator<'a>(
    tokenizer: impl Iterator<Item = tokenizer::Token<'a>>,
) -> impl Iterator<Item = Token<'a>> {
    Aggregator::from(tokenizer)
}

struct Aggregator<'a, I>
where
    I: Iterator<Item = tokenizer::Token<'a>>,
{
    tokenizer: std::iter::Peekable<I>,
}

impl<'a, I> From<I> for Aggregator<'a, I>
where
    I: Iterator<Item = tokenizer::Token<'a>>,
{
    fn from(tokenizer: I) -> Self {
        Self {
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
        macro_rules! peek {
            ($($pattern:pat => $result:expr),+ $(,)?) => {{
                let next_token = self.tokenizer.peek().map(|t| (t.token, t.token_type));
                match next_token {
                    $(
                        $pattern => {
                            self.tokenizer.next();
                            $result
                        }
                    )+
                }
            }};
        }

        loop {
            match self.tokenizer.next() {
                Some(tokenizer::Token { token, token_type }) => {
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
                        tokenizer::TokenType::Numeric => {
                            // Check if this is a floating point number (e.g., 12. or 12.34)
                            if let Some(next) = self.tokenizer.peek() {
                                if next.token_type == tokenizer::TokenType::Punctuation
                                    && next.token == "."
                                {
                                    let start_ptr = token.as_ptr();

                                    // Consume the dot
                                    let dot = self.tokenizer.next().unwrap();

                                    // Check if there's a number after the dot (e.g., 12.34)
                                    let end_ptr = if let Some(after_dot) = self.tokenizer.peek() {
                                        if after_dot.token_type == tokenizer::TokenType::Numeric {
                                            // Consume the fractional part
                                            let frac = self.tokenizer.next().unwrap();
                                            // End pointer is at the end of the fractional part
                                            unsafe { frac.token.as_ptr().add(frac.token.len()) }
                                        } else {
                                            // Just number with trailing dot (e.g., 12.)
                                            unsafe { dot.token.as_ptr().add(dot.token.len()) }
                                        }
                                    } else {
                                        // Just number with trailing dot (e.g., 12.)
                                        unsafe { dot.token.as_ptr().add(dot.token.len()) }
                                    };

                                    // Calculate the full slice from start to end
                                    let len = unsafe { end_ptr.offset_from(start_ptr) as usize };
                                    let float_str = unsafe {
                                        std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                                            start_ptr, len,
                                        ))
                                    };

                                    Token::Number(float_str)
                                } else {
                                    Token::Number(token)
                                }
                            } else {
                                Token::Number(token)
                            }
                        }
                        tokenizer::TokenType::Punctuation => match token {
                            "=" => peek! {
                                Some(("=", _)) => Token::Equals,
                                _ => Token::Assignment,
                            },
                            "!" => peek! {
                                Some((ident, tokenizer::TokenType::Keyword)) => Token::MacroIdentifier(ident),
                                _ => Token::Promotion,
                            },
                            "?" => Token::Coalescence,
                            "@" => Token::Ampersand,
                            ":" => peek! {
                                Some(("=", _)) => Token::ConstantAssignment,
                                _ => Token::Colon,
                            },
                            "." => peek! {
                                Some((".", _)) => Token::DoubleDot,
                                _ => Token::Dot,
                            },
                            "+" => Token::Plus,
                            "-" => peek! {
                                Some(("-", _)) => Token::DoubleMinus,
                                Some((">", _)) => Token::Arrow,
                                _ => Token::Minus,
                            },
                            "*" => peek! {
                                Some(("*", _)) => Token::DoubleAsterisk,
                                _ => Token::Asterisk,
                            },
                            "/" => Token::Slash,
                            "|" => peek! {
                                Some((">", _)) => peek! {
                                    Some((">", _)) => Token::PipeDoubleForward,
                                    _ => Token::PipeForward,
                                },
                                _ => Token::Pipe,
                            },
                            ">" => peek! {
                                Some(("=", _)) => Token::GreaterOrEqual,
                                Some((">", _)) => Token::DoubleGreater,
                                _ => Token::Greater,
                            },
                            "<" => peek! {
                                Some(("=", _)) => Token::LesserOrEqual,
                                Some(("<", _)) => Token::DoubleLesser,
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
                None => break None,
            }
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
