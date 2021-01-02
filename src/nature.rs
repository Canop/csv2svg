use {
    chrono::{
        FixedOffset,
    },
};

#[derive(Debug)]
pub enum Nature {
    //NaiveDate, // not yet supported
    /// we'll take the first offset for the whole column
    Date(FixedOffset),
    Integer,
}

impl Nature {
}

