use chrono::{Datelike, Days, NaiveDate, Weekday};

fn main() {
    let tahun: i32 = 2022;
    let mut c = NaiveDate::from_ymd_opt(tahun, 1, 1).unwrap();
    let mut ganjil: u16 = 0;
    let mut genap: u16 = 0;
    while c.year() == tahun {
        if c.weekday() == Weekday::Sat || c.weekday() == Weekday::Sun {
            genap += 1;
            ganjil += 1;
        }
        else if c.day() % 2 == 0 {
            genap += 1;
        }
        else {
            ganjil += 1;
        }
        c = c.checked_add_days(Days::new(1)).unwrap();
    }
    println!("Jumlah Hari Mobil Pelat GANJIL Boleh Keluar Selama Tahun {} adalah {} Hari", tahun, ganjil);
    println!("Jumlah Hari Mobil Pelat GENAP Boleh Keluar Selama Tahun {} adalah {} Hari", tahun, genap);
}
