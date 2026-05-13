use tokenizer::{Token, tokenize};

static SOURCE: &str = "\
fn greet
    name := \"world\"
    msg := f\"Hello\"
    return msg";

fn main() {
    println!("=== Source ===");
    println!("{SOURCE}");
    println!("\n=== Token stream ===");

    let mut depth = 0usize;
    for result in tokenize(SOURCE) {
        match result {
            Ok(Token::ScopeStart(_)) => {
                println!("{}ScopeStart", "    ".repeat(depth));
                depth += 1;
            }
            Ok(Token::ScopeEnd(_)) => {
                depth = depth.saturating_sub(1);
                println!("{}ScopeEnd", "    ".repeat(depth));
            }
            Ok(Token::Token(ty, pos)) => {
                let text = &SOURCE[pos.source_range];
                println!(
                    "{}{ty:<16}  line {:>2}  col {:>2}  {text:?}",
                    "    ".repeat(depth),
                    pos.line,
                    pos.line_range.start,
                );
            }
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }
}
