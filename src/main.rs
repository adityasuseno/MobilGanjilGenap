use chrono::{Weekday, Duration, NaiveDate};

fn main() {
    let tahun: u16 = 2022;
    let c = NaiveDate::from_ymd_opt(tahun, 0, 1);
    print!("Start Date {}", c)
    c + Duration::days(1);
    println!("Add One day is {}", c)
}
