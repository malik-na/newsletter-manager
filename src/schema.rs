// @generated automatically by Diesel CLI.

diesel::table! {
    newsletters (id) {
        id -> Int4,
        title -> Varchar,
        sender_email -> Varchar,
        sender_name -> Nullable<Varchar>,
        subject -> Varchar,
        content -> Text,
        newsletter_type -> Varchar,
        is_read -> Bool,
        is_favorite -> Bool,
        importance_score -> Float4,
        received_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
