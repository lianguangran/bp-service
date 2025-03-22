// @generated automatically by Diesel CLI.

diesel::table! {
    members (id) {
        id -> Uuid,
        name -> Varchar,
        memo -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    records (id) {
        id -> Uuid,
        member_id -> Uuid,
        systolic -> Int4,
        diastolic -> Int4,
        bmp -> Int4,
        record_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_member (user_id, member_id) {
        user_id -> Uuid,
        member_id -> Uuid,
        is_default -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        openid -> Varchar,
        session_key -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(user_member -> users (user_id));
diesel::joinable!(user_member -> members (member_id));
diesel::joinable!(records -> members (member_id));

diesel::allow_tables_to_appear_in_same_query!(
    members,
    records,
    user_member,
    users,
);
