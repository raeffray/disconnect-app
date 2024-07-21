// @generated automatically by Diesel CLI.

diesel::table! {
    fellows (id) {
        id -> Int4,
        membership_id -> Int4,
        fellowship_type -> Varchar,
    }
}

diesel::table! {
    members (id) {
        id -> Int4,
        membership_id -> Int4,
        name -> Varchar,
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
    roles (id) {
        id -> Int4,
        #[max_length = 255]
        uuid -> Varchar,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    system_user_roles (system_user_id, role_id) {
        system_user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::table! {
    systemusers (id) {
        id -> Int4,
        user_id -> Varchar,
        secret -> Varchar,
    }
}

diesel::joinable!(fellows -> memberships (membership_id));
diesel::joinable!(members -> memberships (membership_id));
diesel::joinable!(system_user_roles -> roles (role_id));
diesel::joinable!(system_user_roles -> systemusers (system_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    fellows,
    members,
    memberships,
    roles,
    system_user_roles,
    systemusers,
);
