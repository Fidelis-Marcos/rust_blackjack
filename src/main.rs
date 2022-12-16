mod baralho;
use crate::baralho::*;
use std::io;

fn main() {
    let mut b:Baralho = Baralho::new(String::from("preto"));
    let mut ais = Carta::new_ctrl(NAIPE::Espada, String::from("As"));
    b.deck.push(ais);
    ais = b.deck.pop().unwrap();
    println!("{:?}", ais);
    // println!("{:?}",b);
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input);
        if  input.trim().eq("a") {
            println!("a");
            break;
        } else if input.trim().eq("b") {
            input.clear();
        }
    }
}
