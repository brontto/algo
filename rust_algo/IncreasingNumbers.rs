use std::io;


fn get_input() -> String {
    
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    buffer
}

fn get_int() -> i64 {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let n: i64 = buffer.trim().parse::<i64>();
    n
} 

fn get_usize() -> usize {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();
    
    let n: usize = buffer.trim().parse::<usize>();
    n
}
//esimerkki
let mut buffer = String::new();
let mut stdin = io::stdin();
stdin.read_to_string(&mut buffer).ok();
let mut words = buffer.as_str().split_ascii_whitespace();

fn main(){
    
    let n = get_usize();

    if n == 1 {
        println!("{}",0);
        return;
    }

    let mut vec: Vec<i64> = Vec::new();

    for _ in 1..n {
        
        let val = get_int();
        vec.push(val);
        
    }
    
    let mut c: i64 = 0;
   
    for i in 1..n {
        
        while vec[i] < vec[i-1] {
            let mut _ve = vec[i];
            _ve += 1;
            c +=1;
        }
    }

    println!("{}",c);
}