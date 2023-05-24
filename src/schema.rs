// @generated automatically by Diesel CLI.

diesel::table! {
    labels (label_id) {
        label_id -> Char,
        user_id -> Char,
        name -> Varchar,
        module -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    project_labels (project_label_id) {
        project_label_id -> Char,
        user_id -> Char,
        project_id -> Char,
        label_id -> Char,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    projects (project_id) {
        project_id -> Char,
        user_id -> Char,
        name -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Char,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(labels -> users (user_id));
diesel::joinable!(project_labels -> labels (label_id));
diesel::joinable!(project_labels -> projects (project_id));
diesel::joinable!(project_labels -> users (user_id));
diesel::joinable!(projects -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(labels, project_labels, projects, users,);
