struct letter(char);

impl letter {
    fn contains(ch: char) -> bool {
        ch.is_alphabetic()
    }
}
