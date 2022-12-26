// mod baralho;
use crate::baralho::*;
use thiserror::Error;
// use std::io;
// use thiserror::Error;

// #[derive(Debug, Error)]
// #[non_exhaustive]
// pub enum GameError {
//     #[error("Not one of the four suits")]
//     InvalidSuit,
// }

pub struct Blackjack {
    pub dealer: Baralho,
    pub jgdr: Vec<Carta>,
    pub mesa: Vec<Carta>,
}

impl Blackjack {
    pub fn start() -> Blackjack {
        let d = Baralho::new(0);
        let j: Vec<Carta> = Vec::new();
        let m: Vec<Carta> = Vec::new();

        Blackjack {
            dealer: d,
            jgdr: j,
            mesa: m,
        }
    }
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
            let valor_mesa = Blackjack::contagem(table.clone());
            table.push(b.deck.pop().unwrap());
            if valor_mesa > 21 {
                println!("{} House Bust", valor_mesa);
                break;
            } else if valor_mao < valor_mesa && valor_mesa <= 21 {
                println!("{} House Wins", valor_mesa);
                break;
            }
        }
    }

    pub fn hit(&self) {
        let temp = self.dealer.deck.pop().unwrap();
        println!("carta pra mao: {:?}", temp);
        self.jgdr.push(temp);
    }
}