pub trait Processor {
    fn process(&self);
    fn type_name(&self) -> &str;
}

pub trait Formattable {
    fn to_json(&self) -> String;
    fn to_xml(&self) -> String;
}

impl<T: Processor> Formattable for T {
    fn to_json(&self) -> String {
        format!("{{\"type\": \"{}\"}}", self.type_name())
    }

    fn to_xml(&self) -> String {
        format!("<{} type=\"{}\"/>", self.type_name(), self.type_name())
    }
}

// Super trait
pub trait Alertable: Processor + Formattable {
    fn must_alert(&self) -> bool;
    fn alert(&self) -> Option<String> {
        if !self.must_alert() {
            return None;
        }
        Some(format!("Alert: {}", self.type_name()))
    }
}

// Trait objects
// dynamic é decidido em tempo de execução
pub fn process_alertables(alertables: &Vec<Box<dyn Alertable>>) {
    for alertable in alertables {
        alertable.process();
    }
}
