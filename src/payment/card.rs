use super::traits::Payment;

pub struct Card {
    pub number: String,
}

impl Payment for Card {
    fn pay(&self) {
        println!("Paying with card! number: {}", self.number)
    }
}
