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
    let mut b: Baralho = Baralho::new(0);
    let mut hand: Vec<Carta> = Vec::new();
    // hand.push(Carta::new_ctrl(NAIPE::Espada, String::from("3")));
    // hand.push(Carta::new_ctrl(NAIPE::Espada, String::from("Q")));
    // hand.push(Carta::new_ctrl(NAIPE::Espada, String::from("K")));
    let mut input = String::new();
    println!("How Many Decks Would you Like?: ");
    let _burner = io::stdin().read_line(&mut input);
    b.baralhos_totais(input.trim().parse::<u32>().unwrap());
    println!("Teste do baralho: {}", b.deck.len());
    b.embaralhar();
    println!("{:?}", b.deck.pop().unwrap());
    loop {
        println!("The Game Begins");
        println!("Would you like to (H)it, (S)tand");
        input.clear();
        let _burner = io::stdin().read_line(&mut input);
        if input.contains("h") {
            let temp = b.deck.pop().unwrap();
            println!("carta pra mao: {:?}",temp);
            hand.push(temp);
        } else if input.contains("s") {
            break;
        }

        // Avaliação das cartas
        let teste = contagem(hand.clone());
        println!("{}", teste);
        if teste > 21 {
            println!("You went Bust. House Wins");
            break;
        }
        // Vitoria/Derrota
    }

    // botar a mesa pra puxar cartas
    let mut table: Vec<Carta> = Vec::new();
    loop {
        table.push(b.deck.pop().unwrap());
        if contagem(table.clone()) > 21 {
            println!("House Loses");
            break;
        } else if contagem(hand.clone()) < contagem(table.clone()) {
            println!("House Loses");
            break;
        } else if contagem(hand.clone()) < contagem(table.clone()) && contagem(table.clone()) <= 21 {
            println!("House Wins");
        }
    }
}
