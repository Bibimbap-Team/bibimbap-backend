CREATE TABLE member (
    id uuid NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    created_at timestamptz NOT NULL
);
