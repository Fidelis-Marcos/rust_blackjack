mod baralho;
use crate::baralho::*;
mod game;
use std::io;

fn main() {
    let mut b: Baralho = Baralho::new(0);
    let mut hand: Vec<Carta> = Vec::new();
    let mut input = String::new();
    let mut bust: bool = false;
    println!("How Many Decks Would you Like?: ");
    let _burner = io::stdin().read_line(&mut input);
    b.baralhos_totais(input.trim().parse::<u32>().unwrap());
    println!("Teste do baralho: {}", b.deck.len());
    b.embaralhar();
    println!("{}", b.deck.pop().unwrap());
    loop {
        println!("The Game Begins");
        let temp = b.deck.pop().unwrap();
        println!("carta pra mao: {:?}", temp);
        hand.push(temp);

        let temp = b.deck.pop().unwrap();
        println!("carta pra mao: {:?}", temp);
        hand.push(temp);
        loop {
            println!("Would you like to (H)it, (S)tand");
            input.clear();
            let _burner = io::stdin().read_line(&mut input);
            if input.contains("h") {
                game::hit(&mut hand, &mut b);
            } else if input.contains("s") {
                break;
            }

            // Avaliação das cartas
            let teste = game::contagem(hand.clone());
            println!("{}", teste);
            if teste > 21 {
                println!("You went Bust. House Wins");
                bust = true;
                break;
            }
            // Vitoria/Derrota
        }

        if !bust {
            game::mesa(game::contagem(hand.clone()), &mut b);
        }
        println!("Would You like to Play again?");
        input.clear();
        let _burner = io::stdin().read_line(&mut input);
        if input.contains("y") {
            hand.clear();
            // table.clear();
            bust = false;
        } else {
            break;
        }
    }
    // botar a mesa pra puxar cartas
}
