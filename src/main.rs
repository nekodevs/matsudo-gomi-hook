pub(crate) mod calendar;
pub(crate) mod gomi;

use chrono::{Local, TimeZone, Weekday};

use crate::calendar::Calendar;

fn main() {
    let calendar = calendar::matsudo::Matsudo::new(Weekday::Thu);
    let today = Local.ymd(2021, 3, 17).and_hms(0, 0, 0);
    let gomi = calendar.gomi_at(today);

    println!(
        "今日 {} は {} の日です",
        today,
        gomi.iter()
            .map(|g| g.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )
}
