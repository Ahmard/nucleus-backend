// @generated automatically by Diesel CLI.

diesel::table! {
    budgets (budget_id) {
        budget_id -> Uuid,
        user_id -> Uuid,
        amount -> Int8,
        amount_used -> Int8,
        month -> Int2,
        year -> Int2,
        title -> Varchar,
        comment -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    expenses (expense_id) {
        expense_id -> Uuid,
        user_id -> Uuid,
        project_id -> Uuid,
        budget_id -> Uuid,
        amount -> Int8,
        narration -> Varchar,
        spent_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    labels (label_id) {
        label_id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        module -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    project_labels (project_label_id) {
        project_label_id -> Uuid,
        user_id -> Uuid,
        project_id -> Uuid,
        label_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    projects (project_id) {
        project_id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
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

diesel::joinable!(budgets -> users (user_id));
diesel::joinable!(expenses -> budgets (budget_id));
diesel::joinable!(expenses -> projects (project_id));
diesel::joinable!(expenses -> users (user_id));
diesel::joinable!(labels -> users (user_id));
diesel::joinable!(project_labels -> labels (label_id));
diesel::joinable!(project_labels -> projects (project_id));
diesel::joinable!(project_labels -> users (user_id));
diesel::joinable!(projects -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    budgets,
    expenses,
    labels,
    project_labels,
    projects,
    users,
);
