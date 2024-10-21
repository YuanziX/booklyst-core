-- +goose Up
CREATE TABLE rentals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    rental_start_date DATE NOT NULL,
    rental_end_date DATE NOT NULL,
    total_rental_cost DECIMAL(10, 2) NOT NULL,
    status TEXT DEFAULT 'ongoing',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (book_id) REFERENCES books(id)
);

CREATE INDEX rentals_user_id_idx ON rentals (user_id);
CREATE INDEX rentals_book_id_idx ON rentals (book_id);
CREATE INDEX rentals_status_idx ON rentals (status);

-- +goose Down
DROP INDEX rentals_user_id_idx;
DROP INDEX rentals_book_id_idx;
DROP INDEX rentals_status_idx;
DROP TABLE rentals;