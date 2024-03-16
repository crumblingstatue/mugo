fn main() {
    let word = std::env::args().nth(1).expect("Need a word");
    eprintln!("Possible root words for '{word}':");
    for root in mugo::deconjugate(&word) {
        eprintln!("{root:?}");
    }
}
