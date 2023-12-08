use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::HandType::HighCard;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Copy, Clone)]
#[derive(Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}


#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Into<Card> for usize {
    fn into(self) -> Card {
        match self {
            0 => Card::Ace,
            1 => Card::King,
            2 => Card::Queen,
            3 => Card::Jack,
            4 => Card::Ten,
            5 => Card::Nine,
            6 => Card::Eight,
            7 => Card::Seven,
            8 => Card::Six,
            9 => Card::Five,
            10 => Card::Four,
            11 => Card::Three,
            12 => Card::Two,
            13 => Card::Joker,
            _ => panic!("Invalid card index."),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

fn get_card_counts(card: &[Card; 5]) -> [u32; 14] {
    let mut card_counts: [u32; 14] = [0; 14];

    for card in card {
        card_counts[*card as usize] += 1;
    }

    card_counts
}

fn get_hand_type(card: &[Card; 5]) -> HandType {
    let card_counts = get_card_counts(card);

    for card_count in card_counts.iter() {
        if *card_count == 5 {
            return HandType::FiveOfAKind;
        }
    }

    for card_count in card_counts.iter() {
        if *card_count == 4 {
            return HandType::FourOfAKind;
        }
    }


    let num_triplets = card_counts.iter().filter(|card_count| **card_count == 3).count();
    let num_pairs = card_counts.iter().filter(|card_count| **card_count == 2).count();

    if num_triplets == 1 && num_pairs == 1 {
        return HandType::FullHouse;
    }

    if num_triplets == 1 {
        return HandType::ThreeOfAKind;
    }

    if num_pairs == 2 {
        return HandType::TwoPair;
    }

    if num_pairs == 1 {
        return HandType::OnePair;
    }

    HighCard
}

fn get_total_winnings(hand_and_bets: &Vec<(Hand, i32)>) -> i32 {
    let total_winnings = hand_and_bets.iter().enumerate()
        .map(|(index, (_, bet))| ((index as i32) + 1, bet))
        .map(|(rank, bet)| rank * bet)
        .sum();
    total_winnings
}

fn get_best_hand_type_from_jokers(hand: &Hand) -> HandType {
    return if let Some((most_frequent_card, _)) =
        get_card_counts(&hand.cards)
            .into_iter()
            .enumerate()
            .map(|(card, card_count)| (card.into(), card_count))
            .filter(|(card, _): &(Card, u32)| *card != Card::Joker)
            .max_by_key(|(_, card_count)| *card_count)
    {
        let cards_with_replaced_jokers = hand.cards.map(|card| if card == Card::Joker { most_frequent_card } else { card });
        get_hand_type(&cards_with_replaced_jokers)
    } else {
        // Only jokers.
        HandType::FiveOfAKind
    };
}

fn parse_hand(cards_str: &str) -> Option<Hand> {
    if cards_str.len() != 5 {
        return None;
    }

    let mut cards: [Card; 5] = [Card::Two; 5];

    for (i, card_str) in cards_str.chars().enumerate() {
        cards[i] = match card_str {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => return None,
        };
    }

    Some(Hand {
        hand_type: get_hand_type(&cards),
        cards,
    })
}

fn parse_card_and_bet(cards_and_bet_str: &str) -> Option<(Hand, i32)> {
    let mut cards_and_bet_split = cards_and_bet_str.split(" ");

    let hand = parse_hand(cards_and_bet_split.next()?)?;
    let bet = cards_and_bet_split.next()?.parse::<i32>().ok()?;

    Some((hand, bet))
}

fn part1(reader: BufReader<File>) -> i32 {
    let mut hand_and_bets: Vec<(Hand, i32)> = reader
        .lines()
        .map(|line| parse_card_and_bet(&line.unwrap()).unwrap()).collect();

    hand_and_bets.sort_by(|(hand1, _), (hand2, _)| hand2.cmp(hand1));

    let total_winnings = get_total_winnings(&hand_and_bets);

    return total_winnings;
}

fn part2(reader: BufReader<File>) -> i32 {
    let mut hand_and_bets: Vec<(Hand, i32)> = reader
        .lines()
        .map(|line| parse_card_and_bet(&line.unwrap()).unwrap())
        .collect();

    hand_and_bets.iter_mut().for_each(|(hand, _)| {
        hand.cards = hand.cards.map(|card| if card == Card::Jack { Card::Joker } else { card });
        hand.hand_type = get_best_hand_type_from_jokers(&hand);
    });

    hand_and_bets.sort_by(|(hand1, _), (hand2, _)| hand2.cmp(hand1));

    let total_winnings = get_total_winnings(&hand_and_bets);

    return total_winnings;
}

fn main() {
    let input_file_name = args().nth(1).expect("Usage: aoc23-7 <input_file_name>");

    let reader_part1 = BufReader::new(File::open(&input_file_name).expect("Cannot open file."));
    let reader_part2 = BufReader::new(File::open(&input_file_name).expect("Cannot open file."));

    println!("Part 1: {}", part1(reader_part1));
    println!("Part 2: {}", part2(reader_part2));
}
