use lexer::{LexKind, lex};

static SOURCE: &str = "\
fn add(x, y)
    return x + y";

fn main() {
    println!("=== Source ===");
    println!("{SOURCE}");
    println!();

    println!("=== Lexed ===");
    for result in lex(SOURCE) {
        let token = result.unwrap();
        match token.kind {
            LexKind::Newline => println!(),
            LexKind::Whitespace(n) => print!("{}", " ".repeat(n)),
            LexKind::Normal(ty) => print!("[{ty}]"),
            LexKind::Brace(brace, dir) => print!("[{brace:?}/{dir:?}]"),
        }
    }
}
