-- Add up migration script here
-- File up

CREATE TABLE IF NOT EXISTS files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    parent_id INTEGER,
    description TEXT,
    s3_key TEXT UNIQUE NOT NULL,
    file_size_bytes INTEGER,
    uploaded_at DATETIME DEFAULT CURRENT_TIMESTAMP

    FOREIGN KEY (parent_id) REFERENCES folders(id) ON DELETE CASCADE
);

CREATE INDEX idx_name ON files(name);
CREATE INDEX idx_key ON files(s3_key);
