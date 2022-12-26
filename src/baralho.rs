use core::fmt::{Display, Formatter};
use rand::prelude::*;
use thiserror::Error;


#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CartaError {
    #[error("Suit not one of 4 suits")]
    InvalidSuit,
}

#[derive(Debug, Clone)]
pub struct Carta {
    pub naipe: NAIPE,
    pub valor: String,
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

    pub fn new_ctrl(n: NAIPE, v: String) -> Result<Carta, CartaError> {
            let saida = match n {
                NAIPE => Ok(Carta::new(n, v)),
                _=> Err(CartaError::InvalidSuit),
            };
            saida
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
        let s = Baralho {
            deck: Vec::new(),
            numero: n,
        };
        // s.gerar();
        s
    }
    pub fn gerar(&mut self) {
        self.deck.push(Carta::new(NAIPE::Paus, String::from("As")));
        for i in 2..11 {
            self.deck.push(Carta::new(NAIPE::Paus, i.to_string()));
        }
        self.deck.push(Carta::new(NAIPE::Paus, String::from("J")));
        self.deck.push(Carta::new(NAIPE::Paus, String::from("Q")));
        self.deck.push(Carta::new(NAIPE::Paus, String::from("K")));

        self.deck.push(Carta::new(NAIPE::Copas, String::from("As")));
        for i in 2..11 {
            self.deck.push(Carta::new(NAIPE::Copas, i.to_string()));
        }
        self.deck.push(Carta::new(NAIPE::Copas, String::from("J")));
        self.deck.push(Carta::new(NAIPE::Copas, String::from("Q")));
        self.deck.push(Carta::new(NAIPE::Copas, String::from("K")));

        self.deck
            .push(Carta::new(NAIPE::Espada, String::from("As")));
        for i in 2..11 {
            self.deck.push(Carta::new(NAIPE::Espada, i.to_string()));
        }
        self.deck.push(Carta::new(NAIPE::Espada, String::from("J")));
        self.deck.push(Carta::new(NAIPE::Espada, String::from("Q")));
        self.deck.push(Carta::new(NAIPE::Espada, String::from("K")));

        self.deck.push(Carta::new(NAIPE::Ouros, String::from("As")));
        for i in 2..11 {
            self.deck.push(Carta::new(NAIPE::Ouros, i.to_string()));
        }
        self.deck.push(Carta::new(NAIPE::Ouros, String::from("J")));
        self.deck.push(Carta::new(NAIPE::Ouros, String::from("Q")));
        self.deck.push(Carta::new(NAIPE::Ouros, String::from("K")));
    }

    // fn adicionar_baralho(&mut self,n:u32) {
    // let bara_adic = Baralho::new(n);
    // bara_adic.gerar();
    // }

    pub fn baralhos_totais(&mut self, n: u32) {
        for i in 0..n {
            let mut burner = Baralho::new(i);
            burner.gerar();
            self.deck.append(&mut burner.deck);
        }
    }

    pub fn embaralhar(&mut self) {
        let mut rng = rand::thread_rng();
        self.deck.shuffle(&mut rng);
    }
}
