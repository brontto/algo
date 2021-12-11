use std::io;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();
    
    let n = buffer.trim().parse::<usize>().unwrap();
    buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();

    let mut nums = buffer.split_whitespace();
    let mut vec = vec![0; n+1];
    let mut pos = vec![0; n+1];
    

    let posmut = &mut pos[..];
    let vecmut = &mut vec[..];
    for i in 1..=n {
        vecmut[i] = nums.next().unwrap().trim().parse::<usize>().unwrap();
        posmut[vecmut[i]] = i;
       
    }


    let mut r = 1;
    for i in 1..=n {
        
        if pos[i-1]>pos[i] {
            r +=1;
        }
    }
    println!("{}",r);


}

