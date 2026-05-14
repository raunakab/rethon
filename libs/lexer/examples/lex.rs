use lexer::{TokenTree, lex};

static SOURCE: &str = "num = rand()

main = fn()
    x = 1
    y = 2
    z =
        if (a = 1; b = 2; a + b < num) x + y
        else throw

    for (a in b)

    result = z ** 2
    result";

fn main() {
    println!("=== Source ===");
    println!("{SOURCE}");
    println!();

    println!("=== Stream ===");

    let mut depth = 0usize;
    for result in lex(SOURCE) {
        let result = result.unwrap();
        match result {
            TokenTree::Start(_) => {
                println!("{}(", "    ".repeat(depth));
                depth = depth.saturating_add(1);
            }
            TokenTree::End(_) => {
                depth = depth.saturating_sub(1);
                println!("{})", "    ".repeat(depth));
            }
            TokenTree::Token(ty, _) => {
                println!("{}[{ty}]", "    ".repeat(depth));
            }
        }
    }
}
