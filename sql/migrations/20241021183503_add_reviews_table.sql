-- +goose Up
CREATE TABLE reviews (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    book_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    rating INTEGER CHECK(rating BETWEEN 1 AND 5),
    comment TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX reviews_book_id_idx ON reviews (book_id);
CREATE INDEX reviews_user_id_idx ON reviews (user_id);

-- +goose Down
DROP INDEX reviews_book_id_idx;
DROP INDEX reviews_user_id_idx;
DROP TABLE reviews;