use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("stdin");
    let sum: i32 = line.split_whitespace().map(|x| x.parse::<i32>().expect("integer")).sum();
    println!("{}", sum);
}