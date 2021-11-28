use std::io;

fn main(){
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");
    let mut n: i64 = n.trim().parse().expect("failed väävää");

    let mut i = 0;

    let mut arr: [i64; n];

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("errorrerrooor");

        if input.is_empty() {break;}
        
        arr[i] = input.trim().parse().expect("failed väävää");
        i +=1;
    
    }

    for elem in arr {
        println!("{}", elem);
    }
}