use std::io;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<usize>().unwrap();

    let sum = n * (n+1) /2;

    if sum % 2 != 0 { 
        println!("NO")
    } else {
            

        let mut divd_sum = sum /2;
        let mut a = Vec::new();
        let mut b = Vec::new();
        for i in (1..=n).rev() {
           if i <= divd_sum {
               a.push(i);
               divd_sum -= i;
               continue;
           } 
           b.push(i);
        }

        println!("YES");
        println!("{}", a.len());
        
        for elem in a {
            print!("{} ", elem)
        }

        println!("\n{}", b.len());
        for elem in b {
            print!("{} ", elem)
        }
         

    }

}
