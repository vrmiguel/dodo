use chrono::{Local, NaiveDate};

pub fn today() -> NaiveDate {
    Local::today().naive_local()
}
