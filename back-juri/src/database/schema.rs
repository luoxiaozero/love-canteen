diesel::table! {
    user (id) {
        id-> Integer,
        account-> Varchar,
        password-> Varchar,
        create_time-> Datetime,
    }
}