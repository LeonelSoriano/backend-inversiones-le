table! {
    business (id) {
        id -> Int4,
        name -> Varchar,
        sign -> Varchar,
    }
}

table! {
    img_products (id) {
        id -> Int4,
        name -> Varchar,
        product_fk -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int4,
        login_timestamp -> Timestamptz,
    }
}

table! {
    people (id) {
        id -> Int4,
        name -> Varchar,
        gender -> Bool,
        age -> Int4,
        address -> Varchar,
        phone -> Varchar,
        email -> Varchar,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        purchase_unit -> Int4,
        retail_unit -> Int4,
        sale_unit -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    products_bussiness_table (id) {
        id -> Int4,
        name -> Varchar,
        product_fk -> Int4,
        business_fk -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    unit (id) {
        id -> Int4,
        name -> Varchar,
        sign -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        address -> Varchar,
        address_number -> Varchar,
        birthday -> Timestamp,
        city -> Varchar,
        email -> Varchar,
        gender -> Bool,
        img -> Nullable<Varchar>,
        password -> Varchar,
        phone -> Varchar,
        username -> Varchar,
        name -> Varchar,
        surname -> Varchar,
        zip -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        login_session -> Varchar,
    }
}

joinable!(img_products -> products (product_fk));
joinable!(login_history -> users (user_id));
joinable!(products_bussiness_table -> business (business_fk));
joinable!(products_bussiness_table -> products (product_fk));

allow_tables_to_appear_in_same_query!(
    business,
    img_products,
    login_history,
    people,
    products,
    products_bussiness_table,
    unit,
    users,
);
