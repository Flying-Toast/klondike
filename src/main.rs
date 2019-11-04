fn main() {
    use klondike::card::*;
    let mut c = Card::new(Suit::Diamonds, Value::Jack);
    c.face_up = true;
    println!("{}", Card::top_row());
    println!("{}", c.ident_row());
    println!("{}", c.empty_row());
    println!("{}", c.empty_row());
    println!("{}", c.empty_row());
    println!("{}", Card::bottom_row());
    let mut d = Card::new(Suit::Diamonds, Value::Jack);
    d.face_up = false;
    println!("{}", d.ident_row());
}
