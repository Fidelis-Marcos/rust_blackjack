mod baralho;
use crate::baralho::*;

// use crate::baralho;

fn main() {
    let c = Carta::new_ctrl(NAIPE::Espadas, String::from("As"));
    println!("THE ACE OF SPADES - THE ACE OF SPADES");
    // println!("{}", c);
    println!("{:?}", c);
}
