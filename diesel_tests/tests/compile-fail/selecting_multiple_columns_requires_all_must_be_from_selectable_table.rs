#[macro_use]
extern crate diesel;

use diesel::*;
use diesel::connection::PgConnection;

table! {
    users {
        id -> Serial,
        name -> VarChar,
    }
}

table! {
    posts {
        id -> Serial,
        title -> VarChar,
        user_id -> Integer,
    }
}

fn main() {
    let connection = PgConnection::establish("").unwrap();
    let stuff = users::table.select((posts::id, posts::user_id));
    //~^ ERROR Selectable
    //~| ERROR E0277
    //~| ERROR E0277
    //~| ERROR E0277
    //~| ERROR E0277
    //~| ERROR E0277
    //~| ERROR E0277
    let stuff = users::table.select((posts::id, users::name));
    //~^ ERROR Selectable
    //~| ERROR E0277
    //~| ERROR E0277
    //~| ERROR E0277
    //~| ERROR E0277
    //~| ERROR E0277
    //~| ERROR E0277
}
