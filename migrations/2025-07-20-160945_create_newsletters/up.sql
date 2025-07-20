-- Create newsletters table with explicit NOT NULL constraints
CREATE TABLE newsletters (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    sender_email VARCHAR NOT NULL,
    sender_name VARCHAR,
    subject VARCHAR NOT NULL,
    content TEXT NOT NULL,
    newsletter_type VARCHAR NOT NULL DEFAULT 'unknown',
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    is_favorite BOOLEAN NOT NULL DEFAULT FALSE,
    importance_score REAL NOT NULL DEFAULT 0.0,
    received_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create index for faster queries
CREATE INDEX idx_newsletters_sender_email ON newsletters(sender_email);
CREATE INDEX idx_newsletters_is_read ON newsletters(is_read);
CREATE INDEX idx_newsletters_received_at ON newsletters(received_at DESC);

