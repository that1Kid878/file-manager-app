-- Add up migration script here
-- Folders up
CREATE TABLE IF NOT EXISTS folders (
    id INTEGER PRIMARY KEY NOT NULL ,
    name TEXT UNIQUE NOT NULL,
    parent_id INTEGER REFERENCES folders(id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_folder_name ON folders(name);
