-- +goose Up
CREATE TABLE payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    order_id INTEGER NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    payment_method TEXT NOT NULL,
    payment_status TEXT CHECK(payment_status IN ('pending', 'completed', 'cancelled')) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (order_id) REFERENCES orders(id)
);

CREATE INDEX payments_user_id_idx ON payments (user_id);
CREATE INDEX payments_order_id_idx ON payments (order_id);

-- +goose Down
DROP INDEX payments_user_id_idx;
DROP INDEX payments_order_id_idx;
DROP TABLE payments;