use core::fmt::{Display, Formatter};
use rand::prelude::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum NAIPE {
    Paus,
    Copas,
    Espada,
    Ouros,
}
impl Display for NAIPE {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use NAIPE::*;
        match self {
            Paus => write!(f, "Paus"),
            Copas => write!(f, "Copas"),
            Espada => write!(f, "Espada"),
            Ouros => write!(f, "Ouros"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Carta {
    naipe: NAIPE,
    valor: String,
}

impl Carta {
    fn new(n: NAIPE, v: String) -> Carta {
        let c: Carta = Carta { naipe: n, valor: v };
        c
    }

    pub fn get_valor(&self) -> &str {
        self.valor.as_str()
    }

    pub fn parsed_valor(&self) -> u32 {
        self.valor.parse::<u32>().unwrap()
    }
}

impl Display for Carta {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} de {}", self.valor, self.naipe)
    }
}

#[derive(Debug)]
pub struct Baralho {
    pub deck: Vec<Carta>,
    pub numero: u32,
}

impl Baralho {
    pub fn new(n: u32) -> Baralho {
        let mut s = Baralho {
            deck: Vec::new(),
            numero: n,
        };
        s.gerar();
        s
    }
    fn gerar(&mut self) {
        for n in NAIPE::iter() {
            self.deck.push(Carta::new(n, String::from("As")));
            for i in 2..11 {
                self.deck.push(Carta::new(n, i.to_string()));
            }
            self.deck.push(Carta::new(n, String::from("J")));
            self.deck.push(Carta::new(n, String::from("Q")));
            self.deck.push(Carta::new(n, String::from("K")));
        }
    }

    pub fn baralhos_totais(&mut self, n: u32) {
        if n > 0 {
            for i in 0..n - 1 {
                let mut burner = Baralho::new(i);
                self.deck.append(&mut burner.deck);
            }
        }
    }

    pub fn embaralhar(&mut self) {
        let mut rng = rand::thread_rng();
        self.deck.shuffle(&mut rng);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tamanho_certo() {
        let mut b = Baralho::new(0);
        assert_eq!(52, b.deck.len());
    }

    #[test]
    fn total_simples() {
        let mut b = Baralho::new(0);
        b.baralhos_totais(0);
        assert_eq!(52, b.deck.len());
    }

    #[test]
    fn total_certo() {
        let mut b = Baralho::new(0);
        b.baralhos_totais(3);
        assert_eq!(156, b.deck.len());
    }
}
