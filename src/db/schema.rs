// diesel::table! {
//     memberships (id) {
//         id -> Int4,
//         code -> Varchar,
//         status_in_platform -> Varchar,
//     }
// }

// diesel::table! {
//     fellows (id) {
//         id -> Int4,
//         membership_id -> Int4,
//         fellowship_type -> Varchar,
//     }
// }

// diesel::joinable!(fellows -> memberships (membership_id));
// diesel::allow_tables_to_appear_in_same_query!(memberships, fellows);
