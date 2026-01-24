struct digit(char);

impl digit {
    fn contains(ch: char) -> bool {
        ch.is_digit()
    }
}
