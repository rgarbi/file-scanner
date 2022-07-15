-- Add migration script here
ALTER TABLE file_scan
    ADD COLUMN scan_result TEXT,
    ADD COLUMN scan_result_details TEXT;
