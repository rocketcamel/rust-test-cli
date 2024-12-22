use super::traits::Payment;

pub struct PaymentContext {
    strategy: Box<dyn Payment>,
}

impl PaymentContext {
    pub fn new(strategy: Box<dyn Payment>) -> Self {
        Self { strategy: strategy }
    }

    pub fn process_payment(&self) {
        self.strategy.pay();
    }
}
