use chrono::{Datelike, NaiveDate};
pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let date = NaiveDate::from_yo_opt(year.try_into().unwrap(), 366);
    match date {
        Some(_) => None,
        None=> NaiveDate::from_ymd_opt(year.try_into().unwrap(), 7, 2).map(|d| d.weekday())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
            println!("{:?}", middle_day(1022));
        }
}
