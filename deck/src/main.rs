use rand::{seq::SliceRandom, thread_rng};

/// 'derive' specifies which traits to implement for this struct
/// #[] specifies an attribute list to apply to the struct
/// by doing this, we are telling the compiler to add all the functions from
/// the Debug trait to the struct
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // tied to struct definition. Called using `::` syntax
    fn new() -> Self {
        let suits = ["Diamond", "Club", "Heart", "Spade"];
        let values = [
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        ];

        let mut cards = Vec::new();
        for suit in suits {
            for value in values {
                let card = format!("{value} of {suit}");
                cards.push(card);
            }
        }
        // This is an implicit return, note the missing semicolon
        Deck { cards }
    }

    // called on an instance of Deck
    // &mut gives us a mutable reference
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        // isize and usize -> hardware dependent max size for indexing collections. Size of a
        // pointer to reference an area in memory
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    println!("Here's the deck: {:#?}", deck);

    deck.shuffle();

    println!("Here's the shuffled deck: {:#?}", deck);

    let hand = deck.deal(3);
    println!("Here's your hand: {:#?}", hand)
}
