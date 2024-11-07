use std::io;
use rand::Rng;

#[derive(Debug)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn value(&self) -> Vec<u32> {
        match self.rank {
            Rank::Two => vec![2],
            Rank::Three => vec![3],
            Rank::Four => vec![4],
            Rank::Five => vec![5],
            Rank::Six => vec![6],
            Rank::Seven => vec![7],
            Rank::Eight => vec![8],
            Rank::Nine => vec![9],
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => vec![10],
            Rank::Ace => vec![1],
        }
    }
}

fn deal_cards_init() {}
fn check_bet(bet: &i32, cash: &i32, totalbets: i32) -> (bool, i32) {
    let mut booly = false;
    if totalbets + bet > *cash {
        println!("You don't have enough cash to bet that much");
        booly = false;
    } else {
        booly = true;
    }
    let max_allowed = cash - totalbets;
    (booly, max_allowed)
}

fn main() {
    println!("Hello, world!");
    let mut handy = String::new();
    println!("How many hands would you like to play?");
    io::stdin()
        .read_line(&mut handy)
        .expect("Failed to read line");
    let numHands: u32 = handy.trim().parse().expect("Please type a number!");

    let mut dealer_cash = 100;
    let mut player_cash = 100;

    //play loop
    loop {
        //init hands
        let mut hands: Vec<Vec<Card>> = Vec::new();
        for _ in 0..numHands {
            hands.push(Vec::new());
        }

        let mut handbets: Vec<i32> = Vec::new();
        let mut totalbets = 0;
        for hand in 0..numHands {
            for _ in 0..2 {
                hands[hand as usize].push(deal_card());
            }
            //init bets

            if hand == 0 {
                continue;
            }

            println!("Hand {} Dealing....", hand);
            println!("Pick a Bet amount for hand {}", hand);
            loop {
                let mut betty = String::new();
                io::stdin()
                    .read_line(&mut betty)
                    .expect("Failed to read line");
                let handbet: i32 = betty.trim().parse().expect("Please type a number!");

                let (boolcheck, maxcheck) = check_bet(&handbet, &player_cash, totalbets);
                if !boolcheck {
                    println!("Max bet allowed: {}", maxcheck);
                    println!("Please type a valid bet");
                } else {
                    handbets.push(handbet);
                    totalbets += handbet;
                    break;
                }
            }
        }

        let exchange: Vec<i32> = play_the_damn_game(&mut hands, &mut handbets);

        for exchang in exchange {
            if exchang > 0 {
                println!("Player Wins: {}", exchang);
                player_cash += exchang;
                dealer_cash -= exchang;
                println!("Dealer Cash after loss: {}", dealer_cash);
                println!("Player cash after win: {}", player_cash);
            } else {
                println!("Dealer Wins: {}", exchang);
                dealer_cash -= exchang;
                player_cash += exchang;
                println!("Player Cash after loss: {}", player_cash);
                println!("Dealer cash after win: {}", dealer_cash);
            }
        }
        println!("Player Cash: {}", player_cash);
        println!("Dealer Cash: {}", dealer_cash);
        println!("Want me to deal you in again? (y/n)");

        let mut again = String::new();
        io::stdin()
            .read_line(&mut again)
            .expect("Failed to read line");
        let againornot: char = again.trim().parse().expect("Please type (y/n)");
        if againornot == 'n' {
            break;
        }
    }
}

fn play_the_damn_game(hands: &mut Vec<Vec<Card>>, handbets: &mut Vec<i32>) -> Vec<i32> {
    let mut hands_left = hands.len() - 1;
    let mut player_hands_vals = Vec::new();
    let mut playerhandwl = Vec::new();
    let mut exchange = Vec::new();
    //init dealer hand to check for bj
    let mut dealer_hand_value = get_hand_value(&hands[0]);
    'outer: loop {
        if dealer_hand_value[0] == 21 {
            println!("Dealer has BlackJack");
            println!("Bummer boys");
            break;
        }
        loop {
            if hands_left == 0 {
                break 'outer;
            }

            for hand in 1..hands.len() {
                let mut hand_value = get_hand_value(&hands[hand]);
                let mut dealer_hand_value = get_hand_value(&hands[0]);
                if hand_value[0] == 21 && hands[hand].len() == 2 {
                    println!("Hand {}: {:?}", hand, hands[hand]);
                    println!("BlackJack!");
                    player_hands_vals.push(hand_value[0]);
                    
                    break;
                }

                loop {
                    if hand_value[0] > 21 {
                        if hand_value.len() > 1 && hand_value[1] <= 21 {
                            hand_value.remove(0);
                        } else {
                            println!("Hand {}: {:?}", hand, hands[hand]);
                            println!("Bust!");
                            player_hands_vals.push(hand_value[0]);
                            hands_left -= 1;
                            break;
                        }
                    } else if hand_value[0] == 21 {
                        println!("Hand {}: {:?}", hand, hands[hand]);
                        println!("Perfect Score");
                        player_hands_vals.push(hand_value[0]);
                        hands_left -= 1;
                        break;
                    } else {
                        println!("HandValue: {:?}", hand_value);
                        println!("hit or stand? (h/s)");
                        println!("Dealer : {:?}", hands[0][0]);
                        println!("Hand {}: {:?}", hand, hands[hand]);

                        let mut player_move = String::new();
                        io::stdin()
                            .read_line(&mut player_move)
                            .expect("Failed to read line");
                        let player_move = player_move.trim();

                        if player_move == "h" {
                            hands[hand].push(deal_card());
                            hand_value = get_hand_value(&hands[hand]);
                        } else if player_move == "s" {
                            player_hands_vals.push(hand_value[0]);
                            hands_left -= 1;
                            break;
                        } else {
                            println!("Invalid move. Please type 'h' for hit or 's' for stand.");
                        }
                    }
                }
            }

            // dealers logic

            loop {
                if dealer_hand_value[0] > 21 {
                    println!("HandValue: {:?}", dealer_hand_value[0]);
                    println!("Hand {:?}: {:?}", 0, hands[0]);
                    println!("Dealer Busts Congrats To Everyone Who Stood!");
                    break;
                } else if dealer_hand_value[0] == 21 {
                    println!("HandValue: {:?}", dealer_hand_value[0]);
                    println!("Hand {}: {:?}", 0, hands[0]);
                    println!("Dealer has a perfect score");
                    break;
                } else if dealer_hand_value[0] >= 17 {
                    println!("HandValue: {:?}", dealer_hand_value[0]);
                    println!("Hand {}: {:?}", 0, hands[0]);
                    println!("Dealer Stands");
                    break;
                } else {
                    hands[0].push(deal_card());
                    dealer_hand_value = get_hand_value(&hands[0]);
                }
            }

            // final showdown
            for i in 0..player_hands_vals.len() {
                println!("Dealers Hand: {:?}", hands[0]);
                if dealer_hand_value[0] > player_hands_vals[i] && player_hands_vals[i] <= 21 && dealer_hand_value[0] <= 21 {
                    println!("Dealer Wins hand {}", i + 1);
                    exchange.push(-handbets[i]);
                } else if dealer_hand_value[0] < player_hands_vals[i] && player_hands_vals[i] <= 21 {
                    println!("Player Wins hand {}", i + 1);
                    exchange.push(handbets[i]);
                } else if dealer_hand_value[0] == player_hands_vals[i] && player_hands_vals[i] <= 21 {
                    println!("Push on hand {}", i + 1);
                    exchange.push(0);
                } else if player_hands_vals[i] > 21 {
                    println!("Dealer Wins hand {}", i + 1);
                    playerhandwl.push(-handbets[i]);
                } else if dealer_hand_value[0] > player_hands_vals[i] && player_hands_vals[i] <= 21 && dealer_hand_value[0] > 21 {
                    println!("Player Wins hand {}", i + 1);
                    exchange.push(handbets[i]);
                } 
                else {
                    println!("zamn thats an edgecase idk wtf");
                }
            }
            break 'outer;
            
        }
    }
    exchange
}

fn get_hand_value(hand: &[Card]) -> Vec<u32> {
    let mut hand_value = vec![0];
    for card in hand {
        for card_value in card.value().iter() {
            if *card_value == 1 {
                let mut new_hand_value = Vec::new();
                for val in &hand_value {
                    new_hand_value.push(val + 1);
                    new_hand_value.push(val + 11);
                }
                hand_value = new_hand_value;
            } else {
                for val in &mut hand_value {
                    *val += card_value;
                }
            }
        }
    }
    hand_value.dedup();
    hand_value.sort_by(|a, b| b.cmp(a));
    hand_value
}

enum Move {
    Hit,
    Stand,
}

fn deal_card() -> Card {
    let mut rng = rand::thread_rng();
    let rank = match rng.gen_range(0..13) {
        0 => Rank::Two,
        1 => Rank::Three,
        2 => Rank::Four,
        3 => Rank::Five,
        4 => Rank::Six,
        5 => Rank::Seven,
        6 => Rank::Eight,
        7 => Rank::Nine,
        8 => Rank::Ten,
        9 => Rank::Jack,
        10 => Rank::Queen,
        11 => Rank::King,
        _ => Rank::Ace,
    };
    let suit = match rng.gen_range(0..4) {
        0 => Suit::Hearts,
        1 => Suit::Diamonds,
        2 => Suit::Clubs,
        _ => Suit::Spades,
    };
    Card { rank, suit }
}