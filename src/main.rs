mod baralho;
use crate::baralho::*;
mod blackjack;
use crate::blackjack::*;
use std::io;

fn main() {
    let mut game = Blackjack::start();
    let mut input = String::new();
    let mut bust: bool = false;
    println!("How Many Decks Would you Like?: ");
    let _burner = io::stdin().read_line(&mut input);
    game.dealer
        .baralhos_totais(input.trim().parse::<u32>().unwrap());
    game.dealer.embaralhar();
    println!("{}", game.dealer.deck.pop().unwrap());
    loop {
        println!("The Game Begins");
        let temp = game.dealer.deck.pop().unwrap();
        println!("carta pra mao: {}", temp);
        game.jgdr.push(temp);

        let temp = game.dealer.deck.pop().unwrap();
        println!("carta pra mao: {}", temp);
        game.jgdr.push(temp);
        loop {
            println!("Would you like to (H)it, (S)tand");
            input.clear();
            let _burner = io::stdin().read_line(&mut input);
            if input.contains("h") {
                game.hit();
            } else if input.contains("s") {
                break;
            }

            // Avaliação das cartas
            let teste = Blackjack::contagem(hand.clone());
            println!("{}", teste);
            if teste > 21 {
                println!("You went Bust. House Wins");
                bust = true;
                break;
            }
            // Vitoria/Derrota
        }

        if !bust {
            Blackjack::mesa(Blackjack::contagem(hand.clone()), &mut b);
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
