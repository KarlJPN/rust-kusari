fn main() {
    let input = String::from("ab\ncd\nef");

    let mut line = 1;
    let mut column = 1;

    for c in input.chars() {
        println!("({}, {}): '{}'", line, column, c);

        if c == '\n' {
            line = line + 1;
            column = 1;
        } else {
            column = column + 1;
        }
    }
}