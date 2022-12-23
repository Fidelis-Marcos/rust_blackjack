// mod baralho;
use crate::baralho::*;
use std::io;
pub fn contagem(h: Vec<Carta>) -> u32 {
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

pub fn mesa(valor_mao: u32, b: &mut Baralho) {
    let mut table: Vec<Carta> = Vec::new();
    loop {
        let valor_mesa = contagem(table.clone());
        table.push(b.deck.pop().unwrap());
        if valor_mesa > 21 {
            println!("{} House Bust", valor_mesa);
            break;
        // } else if contagem(hand.clone()) > contagem(table.clone()) && contagem(table.clone()) < 21 {
        //     println!("House Loses");
        //     break;
        } else if valor_mao < valor_mesa && valor_mesa <= 21 {
            println!("{} House Wins", valor_mesa);
            break;
        }
    }
}

pub fn hit(hand: &mut Vec<Carta>, b: &mut Baralho) {
    let mut input = String::new();
    let _burner = io::stdin().read_line(&mut input);
    let temp = b.deck.pop().unwrap();
    println!("carta pra mao: {:?}", temp);
    hand.push(temp);
}
