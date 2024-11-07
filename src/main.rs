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

fn main() {
    println!("Hello, world!");
    let mut handy = String::new();
    io::stdin()
        .read_line(&mut handy)
        .expect("Failed to read line");
    let numHands: u32 = handy.trim().parse().expect("Please type a number!");
    let mut hands: Vec<Vec<Card>> = Vec::new();
    for _ in 0..numHands {
        hands.push(Vec::new());
    }
    //init hands
    for hand in 0..numHands {
        for _ in 0..2 {
            hands[hand as usize].push(deal_card());
        }
    }
    println!("{:?}", hands);
    play_the_damn_game(hands);
}

fn play_the_damn_game(mut hands: Vec<Vec<Card>>) {
    let mut hands_left = hands.len() - 1;
    let mut player_hands_vals = Vec::new();
    //init dealer hand to check for bj
    let mut dealer_hand_value = get_hand_value(&hands[0]);
    loop {
    if dealer_hand_value[0] == 21 {
        println!("Dealer has BlackJack");
        println!("Bummer boys");
        break;
    }
    loop {
        if hands_left == 0 {
            break;
        }

        for hand in 1..hands.len() {
            let mut hand_value = get_hand_value(&hands[hand]);
            let mut dealer_hand_value = get_hand_value(&hands[0]);

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
            } else if dealer_hand_value[0] < player_hands_vals[i] && player_hands_vals[i] <= 21 {
                println!("Player Wins hand {}", i + 1);
            } else if dealer_hand_value[0] == player_hands_vals[i] && player_hands_vals[i] <= 21 {
                println!("Push on hand {}", i + 1);
            } else if player_hands_vals[i] > 21 {
                println!("hand {} should have waited it out", i + 1);
            
            } else {
                println!("zamn thats an edgecase idk wtf");
            }
        }
    }
}
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