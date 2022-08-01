CREATE TABLE queue_items(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    queue_item_type TEXT NOT NULL,
    queue_item_contents TEXT NOT NULL,
    work_started bigint NOT NULL,
    being_worked bool NOT NULL DEFAULT false,
    error_count int NOT NULL,
    error_message TEXT
);

CREATE INDEX queue_items_being_worked_idx ON queue_items (queue_item_type, being_worked, work_started);