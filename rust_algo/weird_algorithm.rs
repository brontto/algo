use std::io;

fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");
    let mut n: i64 = n.trim().parse().expect("failed väävää");
    
    print!("{:?}",n);



    while n != 1{
        if n%2==0 {
            n = n/2;
        }else{
            n = n*3+1;
        }
        print!(" {}",n);
    }

}
