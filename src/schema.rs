// @generated automatically by Diesel CLI.

diesel::table! {
    inventory (product_id) {
        product_id -> Int4,
        total_quantity -> Int4,
        reserved_quantity -> Int4,
        sold_quantity -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    outbox (id) {
        id -> Int4,
        event_type -> Text,
        payload -> Text,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        th_name -> Text,
        en_name -> Text,
        unit_price -> Float4,
        #[max_length = 255]
        image_path -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(inventory -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(inventory, outbox, products,);
