diesel::table! {
    user (id) {
        id-> Integer,
        account-> Varchar,
        password-> Varchar,
        create_time-> Datetime,
    }
}

diesel::table! {
    shop (id) {
        id-> Integer,
        user_id-> Integer,
        create_time-> Datetime,
    }
}

diesel::table! {
    shop_menu (id) {
        id-> Integer,
        shop_id-> Integer,
        title: String,
        create_time-> Datetime,
    }
}

diesel::table! {
    shop_menu_link_food (id) {
        id-> Integer,
        menu_id-> Integer,
        food_id-> Integer,
        create_time-> Datetime,
    }
}

diesel::table! {
    food (id) {
        id-> Integer,
        title -> Varchar,
        ingredient -> Text,
        method -> Text,
        value -> Text,
        create_time-> Datetime,
    }
}

diesel::table! {
    order (id) {
        id-> Integer,
        user_id-> Integer,
        shop_id-> Integer,
        status -> Varchar,
        reason -> Text,
        remark -> Text,
        reserve_start_time: Datetime,
        reserve_end_time: Datetime,
        create_time-> Datetime,
    }
}

diesel::table! {
    order_food (id) {
        id-> Integer,
        food_id -> Integer,
        food_title -> Varchar,
        food_value -> Text,
        count -> Integer,
        create_time-> Datetime,
    }
}