fn main() {
    let word = std::env::args().nth(1).expect("Need a word");
    eprintln!("Possible root words for '{word}':");
    for root in mugo::deconjugate(&word) {
        eprintln!("{root:?}");
        eprintln!("Dictionary text: {}", root.dict_string());
        eprintln!("Conjugated: {}{}", root.text, root.conjugation_suffix());
    }
}
