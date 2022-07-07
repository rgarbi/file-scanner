-- Add migration script here
ALTER TABLE file_scan
    ADD COLUMN scan_result String,
    ADD COLUMN scan_result_details String;
