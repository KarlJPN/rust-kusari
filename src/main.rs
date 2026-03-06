fn is_passing(score: i64) -> bool {
    score >= 60
}

fn main() {
    let result = is_passing(75);
    println!("{}", result); // true と表示されるはず

    let result2 = is_passing(40);
    println!("{}", result2); // false と表示されるはず

    let result3 = is_passing(60);
    println!("{}", result3);
}