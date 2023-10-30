CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    session_id TEXT NOT NULL,
    name TEXT NOT NULL,
    UNIQUE(session_id)
);
