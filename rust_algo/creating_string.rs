use std::io;
use std::collections::HashSet;


fn main(){
    let mut b = String::new();

    io::stdin().read_line(&mut b).unwrap();
    
    let chars: Vec<char> = b.chars().collect();

    let mut letters = HashSet::new();
    let mut words = HashSet::new();
   
    for elem in chars {
        if !letters.insert(elem) {
            continue;
        }
        
    }
        

}

fn tree (Vec<char> a, Vec<char> b) {
    for i in 0..b.len() {
        
    }
}

