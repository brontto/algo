use std::io;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let n = get_input().trim().parse::<usize>().unwrap();

}
