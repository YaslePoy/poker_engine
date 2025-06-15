use std::borrow::Cow;
use rand::random;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::cmp::{Ordering, PartialEq};
use std::ptr::fn_addr_eq;

pub struct Player {
    name: String,
    score: u32,
    cards: [Card; 2],
}

#[derive(Clone)]
pub struct Card {
    suit: CardSuit,
    rank: u8,
}

impl Card {
    pub fn new(suit: CardSuit, rank: u8) -> Card {
        if rank > 12 {
            panic!("Card rand can't be more than 13")
        }
        Card { suit, rank }
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = vec![];
        for r in 0..13 {
            cards.push(Card::new(CardSuit::Hearts, r));
            cards.push(Card::new(CardSuit::Diamonds, r));
            cards.push(Card::new(CardSuit::Crosses, r));
            cards.push(Card::new(CardSuit::Spades, r));
        }

        cards.shuffle(&mut rand::rng());
        Deck { cards }
    }

    pub fn are_cards_available(&self, players: u8) -> bool {
        self.cards.len() as u8 > (players * 2 + 5)
    }

    pub fn pop_cards_for_player(&mut self) -> [Card; 2] {
        [self.cards.pop().unwrap(), self.cards.pop().unwrap()]
    }

    pub fn open_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

struct Round<'a> {
    participants: Vec<&'a mut Player>,
    opened_cards: Vec<Card>,
    deck: &'a mut Deck,
}

impl<'a> Round<'a> {
    pub fn new(mut participants: Vec<&'a mut Player>, deck: &'a mut Deck) -> Round<'a> {
        for participant_index in 0..participants.len() {
            let participant = &mut participants[participant_index];
            participant.cards = deck.pop_cards_for_player()
        }

        Round {
            participants,
            opened_cards: vec![deck.open_card(), deck.open_card(), deck.open_card()],
            deck,
        }
    }

    pub fn pass_participant(&mut self, player: Player) {
        self.participants.remove(
            self.participants
                .iter()
                .position(|p| -> bool { p.name == player.name })
                .unwrap(),
        );
    }
    pub fn open_card(&mut self) {
        if self.opened_cards.len() == 5 {
            panic!("Can't open more cards, round finished")
        }

        self.opened_cards.push(self.deck.open_card());

        if self.opened_cards.len() == 5 {
            self.finish()
        }
    }

    pub fn finish(&self) {}

    pub fn get_combination(&self, playerIndex: usize) -> CardCombination {
        let player = &self.participants[playerIndex];
        let mut total_hand: Vec<Card> = vec![];
        for card in player.cards.clone() {
            total_hand.push(card);
        }
        for opened_card in &self.opened_cards {
            total_hand.push(opened_card.clone())
        }
        
        CardCombination::Flush(0)
    }

    fn is_street(cards: &[Card]) -> bool {
        let min_rank = cards[0].rank;
        for index in 1..5 {
            if min_rank + index != cards[index as usize].rank {
                return false;
            }
        }
        true
    }
}

pub struct Combination {
    rank: u8,
    combination_rank: u8,
}

pub trait CombinationDeterminer<Combination> {
    fn contains(self, cards: [Card]) -> bool;
    fn get_combination(self, cards: [Card]) -> Combination;
}
#[derive(Clone, PartialEq)]
pub enum CardSuit {
    Hearts,
    Diamonds,
    Crosses,
    Spades,
}
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(PartialEq)]
pub enum CardCombination {
    RoyalFlush(u8),
    FlushStreet(u8),
    Four(u8),
    FullHouse(u8),
    Flush(u8),
    Straight(u8),
    Three(u8),
    TwoPair(u8),
    Pair(u8),
    HighCard(u8),
}

impl CardCombination {
    pub fn combination_rank(&self) -> u8 {
        match self {
            CardCombination::HighCard(_) => 1,
            CardCombination::Pair(_) => 2,
            CardCombination::TwoPair(_) => 3,
            CardCombination::Three(_) => 4,
            CardCombination::Straight(_) => 5,
            CardCombination::Flush(_) => 6,
            CardCombination::FullHouse(_) => 7,
            CardCombination::Four(_) => 8,
            CardCombination::FlushStreet(_) => 9,
            CardCombination::RoyalFlush(_) => 10,
        }
    }
}

impl Ord for CardCombination {
    fn cmp(&self, other: &Self) -> Ordering {
        if {}
    }
}
impl CombinationDeterminer for CardCombination::RoyalFlush {

}

