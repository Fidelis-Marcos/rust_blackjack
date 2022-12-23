use core::fmt;
use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum NAIPE {
    Paus,
    Copas,
    Espada,
    Ouros,
    Invalido,
}
impl fmt::Display for NAIPE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
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

    pub fn new_ctrl(n: NAIPE, v: String) -> Carta {
        if v.len() > 2 {
            Carta::new(NAIPE::Invalido, String::from("Invalido"))
        } else {
            Carta::new(n, v)
        }
    }
}

impl fmt::Display for Carta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.naipe, self.valor)
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
