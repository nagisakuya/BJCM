use black_jack::deck::*;
use black_jack::strategy::*;
use black_jack::*;

pub mod black_jack;
pub mod test;

const BJ_PAYBACK: f32 = 1.5;

fn main() {
    let deck = Deck::new(8);
    let strategy = basic_strategy::BasicStrategy8::new();

    let mut profit = 0.0;
    for _ in 0..10000 {
        profit += play_on_strategy(&mut deck.clone(), &strategy)
    }
    println!("{}", profit);
}

fn play_on_strategy(deck: &mut Deck, strategy: &dyn Strategy) -> f32 {
    let mut dealer = Dealer::create(deck);
    let mut vec: Vec<Player> = Vec::new();
    vec.push(Player::create(deck));

    let (_,dealer_bj) = dealer.status();

    if !dealer_bj {
        for i in 0.. {
            if !(i < vec.len()) {
                break;
            }
            loop {
                match strategy.get(&vec[i], &dealer) {
                    Action::Hit => vec[i].hit(deck),
                    Action::Stand => break,
                    Action::Split => {
                        let (new_hand, flag) = vec[i].split(deck);
                        vec.push(new_hand);
                        if flag {
                            break;
                        }
                    }
                    Action::Double => {
                        vec[i].double(deck);
                        break;
                    }
                    Action::Surrender => {
                        vec[i].surrender();
                        break;
                    }
                }
            }
        }
    }

    dealer.drow(deck);
    let mut profit = 0.0;
    for item in vec.iter() {
        profit += match black_jack::judge(item, &dealer) {
            BJResult::BJ => BJ_PAYBACK,
            BJResult::Win => 1.0,
            BJResult::Push => 0.0,
            BJResult::Lose => -1.0,
            BJResult::Surrender => -0.5,
        } * if item.doubled { 2.0 } else { 1.0 };
    }
    profit
}
