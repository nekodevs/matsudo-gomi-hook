use std::fmt::{Display, Formatter, Result};

pub(crate) struct Gomi {
    name: String,
}

impl Gomi {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Display for Gomi {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}
