mod baralho;
use crate::baralho::*;
use std::io;

fn contagem(h: Vec<Carta>) -> u32 {
    let mut s: u32 = 0;
    let mut wait: u32 = 0;
    for c in h {
        match c.get_valor() {
            "j" | "J" | "q" | "Q" | "k" | "K" => s += 10,
            "As" => wait += 1,
            _ => s += c.parsed_valor(),
        }
    }
    while wait > 0 {
        let teste = s + (wait * 10);
        if teste > 21 {
            s += 1;
            wait -= 1;
        } else {
            s += 10;
            wait -= 1;
        }
    }
    s
}

fn main() {
    let mut b: Baralho = Baralho::new(String::from("preto"));
    let mut hand: Vec<Carta> = Vec::new();
    hand.push(Carta::new_ctrl(NAIPE::Espada, String::from("As")));
    hand.push(Carta::new_ctrl(NAIPE::Espada, String::from("As")));
    hand.push(Carta::new_ctrl(NAIPE::Espada, String::from("As")));
    b.gerar();
    println!("{}", b.deck.len());
    b.embaralhar();
    println!("{:?}", b.deck.pop().unwrap());
    let mut input = String::new();
    loop {
        println!("The Game Begins");

        // Avaliação das cartas
        let teste = contagem(hand);
        println!("{}", teste);

        // Vitoria/Derrota

        let _burner = io::stdin().read_line(&mut input);
        break;
    }
}
