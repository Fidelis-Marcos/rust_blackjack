use crate::baralho::*;

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

    pub fn cont_jgdr(&self) -> u32 {
        let s = Blackjack::contagem(self.jgdr.clone());
        s
    }

    pub fn cont_mesa(&mut self) -> bool {
        let valor_mao = Blackjack::cont_jgdr(&self);
        loop {
            let valor_mesa = Blackjack::contagem(self.mesa.clone());
            self.mesa.push(self.dealer.deck.pop().unwrap());
            if valor_mesa > 21 {
                return false;
            } else if valor_mao < valor_mesa && valor_mesa <= 21 {
                return true;
            }
        }
    }

    pub fn hit(&mut self) {
        let temp = self.dealer.deck.pop().unwrap();
        println!("Sua carta é um {}", temp);
        self.jgdr.push(temp);
    }
}
