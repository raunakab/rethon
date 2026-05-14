use scoper::{Token, scope};

static SOURCE: &str = "
num = rand()

fn main
    x = 1
    y = 2
    z =
        if x + y > num
            x + y
        else
            throw

    result = z ** 2
    result
";

fn main() {
    println!("=== Source ===");
    println!("{SOURCE}");
    println!();

    println!("\n=== Token stream ===");

    let mut depth = 0usize;
    for result in scope(SOURCE) {
        let result = result.unwrap();
        match result {
            Token::ScopeStart(_) => {
                println!("{}{{", "    ".repeat(depth));
                depth = depth.saturating_add(1);
            }
            Token::ScopeEnd(_) => {
                depth = depth.saturating_sub(1);
                println!("{}}}", "    ".repeat(depth));
            }
            Token::Token(ty, _) => {
                println!("{}{ty}", "    ".repeat(depth));
            }
        }
    }
}
