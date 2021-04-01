use crate::calendar::Calendar;
use crate::gomi::Gomi;

use chrono::{DateTime, Datelike, Local, Weekday, Weekday::*};

// Defines the gomi rules for Matsudo using a macro.
macro_rules! matsudo_rules {
    (
        $current_base: expr,
        $current_time: expr,
        // This repetition represents each rules based on the base day.
        $((
            $base: path,
            ($burnable1: path, $burnable2: path, $burnable3: path),
            ($glass_week_num: expr, $glass_week_day: path),  // The Nth day of X
            $recycle_plastic: path,
            $other_plastic: path
        )),*
        $(,)?
    ) => {
        // Matches the current base day for the location.
        match ($current_base) {
            $(
                $base => (vec![
                    match ($current_time.weekday()) {
                        $base => Some(Gomi::new("資源ごみ")),
                        $burnable1 | $burnable2 | $burnable3 => Some(Gomi::new("燃やせるごみ")),
                        $recycle_plastic => Some(Gomi::new("リサイクルするプラスチック")),
                        $other_plastic => Some(Gomi::new("その他のプラスチックなどのごみ")),
                        _ => None,
                    },
                    if (week_num_by_weekday($current_time, $glass_week_day) == $glass_week_num) {
                        Some(Gomi::new("陶磁器・ガラスなどのごみ"))
                    } else {
                        None
                    }
                ]
                    // Converts Vec<Option<Gomi>> to Vec<Gomi>, remaining only Some elements.
                    .into_iter()
                    .flat_map(|x| x)
                    .collect()
                )
            ),*,
            // Ignores the unknown base day, returning an empty Vec.
            _ => vec![],
        }
    }
}

pub(crate) struct Matsudo {
    base: Weekday,
}

impl Matsudo {
    pub(crate) fn new(base: Weekday) -> Self {
        Self { base }
    }
}

impl Calendar for Matsudo {
    fn gomi_at(&self, time: DateTime<Local>) -> Vec<Gomi> {
        // Using the defined rules, determines which Gomi's are available to take out.
        matsudo_rules!(
            self.base,
            time,
            (Mon, (Tue, Thu, Sat), (2, Thu), Wed, Fri),
            (Tue, (Mon, Wed, Fri), (2, Wed), Thu, Sat),
            (Wed, (Tue, Thu, Sat), (3, Thu), Fri, Mon),
            (Thu, (Mon, Wed, Fri), (3, Wed), Sat, Tue),
            (Fri, (Tue, Thu, Sat), (4, Thu), Mon, Wed),
            (Sat, (Mon, Wed, Fri), (4, Wed), Tue, Thu),
        )
    }
}

fn week_num_by_weekday(time: DateTime<Local>, weekday: Weekday) -> u32 {
    time.day() / 7
        + if time.weekday().num_days_from_sunday() < weekday.num_days_from_sunday() {
            0
        } else {
            1
        }
}
