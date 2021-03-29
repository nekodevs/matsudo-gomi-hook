use chrono::{DateTime, Local};

use crate::gomi::Gomi;

pub(crate) mod matsudo;

pub(crate) trait Calendar {
    fn gomi_at(&self, time: DateTime<Local>) -> Vec<Gomi>;
}
