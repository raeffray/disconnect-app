// @generated automatically by Diesel CLI.

diesel::table! {
    fellows (id) {
        id -> Int4,
        membership_id -> Int4,
        fellowship_type -> Varchar,
    }
}

diesel::table! {
    memberships (id) {
        id -> Int4,
        code -> Varchar,
        status_in_platform -> Varchar,
    }
}

diesel::table! {
    members (id) {
        id -> Int4,
        name -> Varchar,
        membership_id -> Int4
    }
}

diesel::joinable!(members -> memberships (membership_id));
diesel::joinable!(fellows -> memberships (membership_id));

diesel::allow_tables_to_appear_in_same_query!(memberships, members, fellows);
