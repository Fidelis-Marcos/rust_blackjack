mod baralho;
use crate::baralho::*;

// use crate::baralho;

fn main() {
    let b:Baralho = Baralho::new(String::from("preto"));
    println!("{:?}",b);
}
