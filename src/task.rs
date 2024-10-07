use diesel::{self, prelude::*};

mod schema {
    use diesel::table;

    table! {
        tasks {
            id -> Integer,
            title -> Text,
            done -> Bool,
        }
    }
}

pub struct Member {
    pub id: String,
    pub name: String
}
