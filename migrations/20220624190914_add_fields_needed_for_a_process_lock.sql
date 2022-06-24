-- Add migration script here
ALTER TABLE file_scan
    ADD COLUMN being_worked bool NOT NULL DEFAULT false,
    ADD COLUMN work_started int;
CREATE INDEX status_being_worked_idx ON file_scan (being_worked, work_started, status);