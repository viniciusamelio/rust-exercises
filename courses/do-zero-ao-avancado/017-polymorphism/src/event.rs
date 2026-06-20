use crate::traits::Processor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event<T> {
    pub payload: T,
    pub timestamp: String,
}

impl<T: Processor> Event<T> {
    pub fn new(payload: T, timestamp: String) -> Self {
        Self { payload, timestamp }
    }

    pub fn process(&self) {
        self.payload.process()
    }
}
