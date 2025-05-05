use rand::{rng,seq::SliceRandom};
#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}
impl Deck{
    fn new()-> Self{
    
    let suits= vec!["Hearts", "Diamonds", "Clubs", "Spades"];
    let values = ["Ace", "Two", "Three"];
    let mut cards = vec![];
    for suit in suits
    {
        for value in values{
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }
    let deck = Deck { cards };
    deck
    }
    fn shuffle(&mut self)
    {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }
    fn deal(&mut self , num_cards :usize) -> Vec<String>
    {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}
fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("Heres your deck: {:#?}" , deck);
}
