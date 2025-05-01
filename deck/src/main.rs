#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}
fn main() {
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
    println!("Heres your deck: {:#?}" , deck);
}
