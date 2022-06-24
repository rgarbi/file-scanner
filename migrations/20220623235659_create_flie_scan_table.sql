-- Add migration script here
CREATE TABLE file_scan(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    file_name TEXT NOT NULL,
    file_location TEXT NOT NULL,
    file_hash TEXT NOT NULL,
    posted_on timestamptz NOT NULL,
    last_updated timestamptz NOT NULL,
    status TEXT NOT NULL
);

CREATE UNIQUE INDEX file_hash_idx ON file_scan (file_hash);