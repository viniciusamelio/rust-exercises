use time::{PrimitiveDateTime as DateTime, ext::NumericalDuration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let one_gigasecond: i64 = 1000000000;
    let result = start.checked_add(one_gigasecond.seconds());
    result.unwrap()
}
