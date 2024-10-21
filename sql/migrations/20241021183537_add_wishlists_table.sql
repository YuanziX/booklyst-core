-- +goose Up
CREATE TABLE wishlists (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
);

CREATE INDEX wishlists_user_id_idx ON wishlists (user_id);

-- +goose Down
DROP INDEX wishlists_user_id_idx;
DROP INDEX wishlists_book_id_idx;
DROP TABLE wishlists;