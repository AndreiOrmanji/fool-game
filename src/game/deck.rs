use playin_cards::Card;
use rand::{seq::SliceRandom, Rng};

pub struct Deck {
    cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        Self {
            cards: playin_cards::gen_shoe(1, false),
        }
    }
}

impl From<Vec<Card>> for Deck {
    fn from(cards: Vec<Card>) -> Self {
        Self { cards }
    }
}

impl Deck {
    pub fn new(with_jokers: bool) -> Self {
        Self {
            cards: playin_cards::gen_shoe(1, with_jokers),
        }
    }

    pub fn shuffle<R>(mut self, rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        self.cards.shuffle(rng);

        self
    }

    pub fn new_shuffled<R>(with_jokers: bool, rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        Deck::new(with_jokers).shuffle(rng)
    }

    pub fn take_from_top(&mut self, n_cards: usize) -> Vec<Card> {
        self.take(n_cards, true)
    }

    pub fn take_from_bottom(&mut self, n_cards: usize) -> Vec<Card> {
        self.take(n_cards, false)
    }

    fn take(&mut self, n_cards: usize, from_top: bool) -> Vec<Card> {
        match self.cards.len() {
            0 => Vec::with_capacity(0),
            l @ _ => {
                let range = if l <= n_cards {
                    0..l
                } else if from_top {
                    l - n_cards..l
                } else {
                    0..n_cards
                };

                self.cards.drain(range).collect()
            }
        }
    }
}
