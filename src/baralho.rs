#[derive(Debug)]
pub enum NAIPE {
    Paus,
    Copas,
    Espadas,
    Ouros,
    Invalido,
}
#[derive(Debug)]
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

#[derive(Debug)]
pub struct Baralho {
    deck: Vec<Carta>,
    cor: String,
}

impl Baralho {
    fn new(c: String) -> Baralho{
        let d:Vec<Carta> = Vec::new();
        let mut saida= Baralho {deck: d, cor: c};
        saida.deck.push(Carta::new(NAIPE::Espadas, String::from("As")));

        saida

    }
}
