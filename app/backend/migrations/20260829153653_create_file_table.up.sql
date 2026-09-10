-- Add up migration script here
-- File up

CREATE TABLE IF NOT EXISTS files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    parent_id INTEGER REFERENCES folders(id),
    description TEXT,
    s3_key TEXT UNIQUE NOT NULL,
    file_size_bytes INTEGER,
    uploaded_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_file_name ON files(name);
CREATE INDEX idx_file_key ON files(s3_key);
