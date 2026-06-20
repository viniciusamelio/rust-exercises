pub mod event;
pub mod payload;
pub mod traits;

pub use event::*;
pub use payload::*;
pub use traits::*;

#[cfg(test)]
mod tests {
    use super::*;
    use event::Event;
    use payload::LogPayload;

    #[test]
    fn test_event_process() {
        let event = Event::new(
            LogPayload {
                message: "test".to_string(),
                level: "info".to_string(),
            },
            "2021-01-01T00:00:00Z".to_string(),
        );
        event.process();
        assert_eq!(event.payload.type_name(), "log");
    }

    #[test]
    fn test_formattable() {
        let payload = LogPayload {
            message: "test".to_string(),
            level: "info".to_string(),
        };
        assert_eq!(payload.to_json(), r#"{"type": "log"}"#);
        assert_eq!(payload.to_xml(), r#"<log type="log"/>"#);
    }

    #[test]
    fn test_alertable() {
        let payload = LogPayload {
            message: "test".to_string(),
            level: "info".to_string(),
        };
        assert_eq!(payload.alert(), None);

        let payload = LogPayload {
            message: "test".to_string(),
            level: "error".to_string(),
        };
        assert_eq!(payload.alert(), Some("test".to_string()));
    }

    #[test]
    fn test_process_alertables() {
        let alertables: Vec<Box<dyn Alertable>> = vec![
            Box::new(LogPayload {
                message: "test".to_string(),
                level: "info".to_string(),
            }),
            Box::new(LogPayload {
                message: "test".to_string(),
                level: "error".to_string(),
            }),
        ];
        process_alertables(&alertables);
    }
}
