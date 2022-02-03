use std::io;
use std::collections::HashSet;

fn main(){

    let mut string = String::new();

    io::stdin().read_line(&mut string).unwrap();
    
    string = string.trim().to_string();

    let len: usize = string.len();   
    
    let mut perm: HashSet<String> = HashSet::new();

    generate(&mut perm, string.clone(), 0, len);
    let mut permu = perm.into_iter().collect::<Vec<_>>();
    
    permu.sort();
    println!("{}", permu.len());
    for p in permu {
        
        println!("{}", p);
    }
}

fn generate(perm: &mut HashSet<String>, string: String, start: usize, end: usize){
    

    if start == end-1 {
        perm.insert(string);
        
    }else{
        let mut i = start;
        while i < end {
            let string_new = swap(string.clone(), start, i);
            
            generate(perm, string_new, start+1, end);
            i += 1;
        }
    }


}

fn swap (string: String, i: usize, j: usize) -> String{
    let mut chars: Vec<char> = string.clone().chars().collect();
    let temp = chars[i];
    chars[i] = chars[j];
    chars[j] = temp;

    let s: String = chars.into_iter().collect();

    s
}

