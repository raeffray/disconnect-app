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

diesel::table! {
    systemusers (id) {
        id -> Int4,
        user_id -> Varchar,
        secret -> Varchar
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    system_user_roles (system_user_id, role_id) {
        system_user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    systemusers,
    system_user_roles,
);

diesel::joinable!(system_user_roles -> roles (role_id));
diesel::joinable!(system_user_roles -> systemusers (system_user_id));