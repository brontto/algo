use std::io;


fn get_input() -> String {
    
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    buffer
}

fn main(){
    
    let n = get_input().trim().parse::<usize>().unwrap();

    if n == 1 {
        println!("{}",0);
        return;
    }

    let mut vec: Vec<i64> = Vec::new();
    
    let mut buffer = String::new();
    let stdin = io::stdin();
    
    stdin.read_line(&mut buffer).ok();
    
    let mut words = buffer.split_whitespace();
    
    
    for _ in 0..n {
        vec.push(words.next().unwrap().trim().parse::<i64>().unwrap());
    }
    
    let c = how_much_increasing(&mut vec);

    println!("{}",c);
}

fn how_much_increasing(vec: &mut Vec<i64>) -> i64 {
    let mut c: i64 = 0;
    for i in 1..vec.len() {
        while vec[i] < vec[i-1] {
            c += vec[i-1] - vec[i];
            vec[i] = vec[i-1];
        }
    }
    c
}