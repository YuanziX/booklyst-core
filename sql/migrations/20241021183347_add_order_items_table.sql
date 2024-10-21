-- +goose Up
CREATE TABLE order_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    quantity INTEGER DEFAULT 1,
    price DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES orders(id),
    FOREIGN KEY (book_id) REFERENCES books(id)
);

CREATE INDEX order_items_order_id_idx ON order_items (order_id);
CREATE INDEX order_items_book_id_idx ON order_items (book_id);

-- +goose Down
DROP INDEX order_items_order_id_idx;
DROP INDEX order_items_book_id_idx;
DROP TABLE order_items;