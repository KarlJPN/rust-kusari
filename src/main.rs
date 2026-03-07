enum TokenKind {    // *TokenKind*に下記のラベルを定義した
    Number(i64),    // 整数のラベル
    Plus,           // +のラベル
}

struct Position {   // *Postion*に下記の構造を定義した
    line: usize,    // line(行)に0以上の整数型を定義
    column: usize,  // column(文字)に0以上の整数型を定義
}

struct Token {          // *Token*に下記の構造を定義した
    kind: TokenKind,    // kind変数にTokenKindを定義
    position: Position, // position変数にPostionを定義
}

fn main() {
    let token = Token {     // 変数tokenにTokenそのものを束縛
        kind: TokenKind::Number(42),    // Number(i64)の整数のラベルが貼られているのでそれをkindに束縛
        position: Position { line: 1, column: 5},   // Positionに具体的なusizeの整数型の行と○文字目を定義
    };

    println!("{}行目の{}文字目", token.position.line, token.position.column);   // 変数tokenのposionにあるline:1を取り出して、次にcolumnを取り出しプレースホルダーにそれぞれ置換
    match token.kind {  // match関数で変数tokenのkindで条件判定させる
        TokenKind::Number(n) => println!("数値: {}", n),    // TokenKind::Number(42)がkindに束縛されているので数値: 42を表示
        TokenKind::Plus => println!("演算子: +"),   // TokenKind::Plusは現時点で何も定義されていないので無視される
    }
}