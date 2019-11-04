fn main() {
    use klondike::card::*;
    let c = Card::new(Suit::Diamonds, Value::Jack);
    println!("{}", Card::top_row());
    println!("{}", c.ident_row());
    println!("{}", Card::empty_row());
    println!("{}", Card::empty_row());
    println!("{}", Card::empty_row());
    println!("{}", Card::bottom_row());
}
