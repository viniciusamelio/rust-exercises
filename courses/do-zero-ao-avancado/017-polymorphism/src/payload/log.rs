use crate::{Alertable, Processor};

pub struct LogPayload {
    pub message: String,
    pub level: String,
}

impl Processor for LogPayload {
    fn process(&self) {
        println!("{}", self.message)
    }

    fn type_name(&self) -> &str {
        "log"
    }
}

impl Alertable for LogPayload {
    fn must_alert(&self) -> bool {
        self.level == "error"
    }

    fn alert(&self) -> Option<String> {
        if !self.must_alert() {
            return None;
        }
        Some(self.message.clone())
    }
}
