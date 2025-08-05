pub trait Capitalizable {
    fn ascii_capitalized(&self) -> String;
}

impl Capitalizable for String {
    fn ascii_capitalized(&self) -> String {
        let mut new_string = self.clone();

        if let Some(first_letter) = new_string.get_mut(0..1) {
            first_letter.make_ascii_uppercase();
        }

        new_string
    }
}

impl Capitalizable for &str {
    fn ascii_capitalized(&self) -> String {
        self.to_string().ascii_capitalized()
    }
}