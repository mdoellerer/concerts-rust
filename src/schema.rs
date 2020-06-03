table! {
    artists (id) {
        id -> Int8,
        name -> Varchar,
        country -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    concert_types (id) {
        id -> Int8,
        description -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    concerts (id) {
        id -> Int8,
        concert_date -> Date,
        setlist -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        concert_type_id -> Int4,
        artist_id -> Int8,
        venue_id -> Int8,
    }
}

table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        email -> Varchar,
        email_verified_at -> Nullable<Timestamp>,
        password -> Varchar,
        remember_token -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        api_token -> Nullable<Varchar>,
    }
}

table! {
    venues (id) {
        id -> Int8,
        name -> Varchar,
        city -> Varchar,
        country -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(concerts -> artists (artist_id));
joinable!(concerts -> concert_types (concert_type_id));
joinable!(concerts -> venues (venue_id));

allow_tables_to_appear_in_same_query!(
    artists,
    concert_types,
    concerts,
    users,
    venues,
);
