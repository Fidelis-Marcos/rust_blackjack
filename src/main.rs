mod baralho;
use crate::baralho::*;
use std::io;
// use rand::prelude::*;

fn main() {
    let mut b:Baralho = Baralho::new(String::from("preto"));
    b.gerar();
    println!("{}", b.deck.len());
    b.embaralhar();
    println!("{:?}", b.deck.pop().unwrap());
    let mut input = String::new();
    loop {
        let _burner = io::stdin().read_line(&mut input);
        if  input.trim().eq("a") {
            println!("a");
            break;
        } else if input.trim().eq("b") {
            input.clear();
        }
    }
}
