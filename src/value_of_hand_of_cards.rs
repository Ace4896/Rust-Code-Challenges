#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        use Card::*;

        let mut total = 0;
        let mut aces = 0;

        for card in &self.cards {
            total += match card {
                Two => 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
                Seven => 7,
                Eight => 8,
                Nine => 9,
                Jack | Queen | King => 10,
                Ace => {
                    aces += 1;
                    continue;
                }
            };
        }

        for _ in 0..aces {
            // If total + 11 > 21, value is 1
            // Otherwise, value is 11
            total += if total <= 10 { 11 } else { 1 };
        }

        total
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_hand() {
        let hand = Hand::new();

        assert_eq!(hand.value(), 0);
    }

    #[test]
    fn strong_hand() {
        let mut hand = Hand::new();
        hand.add(Card::Queen);
        hand.add(Card::Ace);

        assert_eq!(hand.value(), 21);
    }

    #[test]
    fn risky_hand() {
        let mut hand = Hand::new();
        hand.add(Card::King);
        hand.add(Card::Queen);
        hand.add(Card::Ace);

        assert_eq!(hand.value(), 21);
    }

    #[test]
    fn oops() {
        let mut hand = Hand::new();
        hand.add(Card::King);
        hand.add(Card::Seven);
        hand.add(Card::Five);

        assert!(hand.is_loosing_hand());
        assert_eq!(hand.value(), 22);
    }
}
