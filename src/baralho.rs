use core::fmt;

#[derive(Debug, Copy, Clone)]
pub enum NAIPE {
    Paus,
    Copas,
    Espada,
    Ouros,
    Invalido,
}
 impl fmt::Display for NAIPE {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) ->fmt::Result {
        write!(f, "{}", self)
    }
 }


#[derive(Debug,Clone)]
pub struct Carta {
    naipe: NAIPE,
    valor: String,
}

impl Carta {
    fn new(n: NAIPE, v: String) -> Carta {
        let c: Carta = Carta { naipe: n, valor: v };
        c
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
    pub cor: String,
}

impl Baralho {
    pub fn new(c: String) -> Baralho{
        let s = Baralho{
            deck: Vec::new(),
            cor: c,
        };
        s
    }
    pub fn gerar(&mut self) {
        self.deck.push(Carta::new(NAIPE::Paus, String::from("As")));
        for i in 2..10 {
            self.deck.push(Carta::new(NAIPE::Paus, i.to_string()));
        }
        self.deck.push(Carta::new(NAIPE::Paus, String::from("J")));
        self.deck.push(Carta::new(NAIPE::Paus, String::from("Q")));
        self.deck.push(Carta::new(NAIPE::Paus, String::from("K")));

        self.deck.push(Carta::new(NAIPE::Copas, String::from("As")));
        for i in 2..10 {
            self.deck.push(Carta::new(NAIPE::Copas, i.to_string()));
        }
        self.deck.push(Carta::new(NAIPE::Copas, String::from("J")));
        self.deck.push(Carta::new(NAIPE::Copas, String::from("Q")));
        self.deck.push(Carta::new(NAIPE::Copas, String::from("K")));

        self.deck.push(Carta::new(NAIPE::Espada, String::from("As")));
        for i in 2..10 {
            self.deck.push(Carta::new(NAIPE::Espada, i.to_string()));
        }
        self.deck.push(Carta::new(NAIPE::Espada, String::from("J")));
        self.deck.push(Carta::new(NAIPE::Espada, String::from("Q")));
        self.deck.push(Carta::new(NAIPE::Espada, String::from("K")));

        self.deck.push(Carta::new(NAIPE::Ouros, String::from("As")));
        for i in 2..10 {
            self.deck.push(Carta::new(NAIPE::Ouros, i.to_string()));
        }
        self.deck.push(Carta::new(NAIPE::Ouros, String::from("J")));
        self.deck.push(Carta::new(NAIPE::Ouros, String::from("Q")));
        self.deck.push(Carta::new(NAIPE::Ouros, String::from("K")));

        
    }
}
