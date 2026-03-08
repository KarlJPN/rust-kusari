enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Identifier(String),
}

struct Position {
    line: usize,
    column: usize,
}

struct Token {
    kind: TokenKind,
    position : Position,
}

fn main() {
    let token = TokenKind::Number(42);
    
}