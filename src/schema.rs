// @generated automatically by Diesel CLI.

diesel::table! {
    newsletters (id) {
        id -> Int4,
        title -> Varchar,
        sender_email -> Varchar,
        sender_name -> Nullable<Varchar>,
        subject -> Varchar,
        content -> Text,
        newsletter_type -> Nullable<Varchar>,
        is_read -> Nullable<Bool>,
        is_favorite -> Nullable<Bool>,
        importance_score -> Nullable<Float4>,
        received_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}
