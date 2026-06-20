use crate::Processor;

pub struct MetricPayload {
    pub name: String,
    pub value: f64,
}

impl Processor for MetricPayload {
    fn process(&self) {
        println!("{}: {}", self.name, self.value)
    }

    fn type_name(&self) -> &str {
        "metric"
    }
}
