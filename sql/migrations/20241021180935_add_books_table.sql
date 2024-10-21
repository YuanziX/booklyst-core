-- +goose Up
CREATE TABLE books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL,
    available_stock INTEGER DEFAULT 0,
    is_rentable BOOLEAN DEFAULT 1,
    rental_price_per_day DECIMAL(10, 2),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX books_title_idx ON books (title);
CREATE INDEX books_author_idx ON books (author);
CREATE INDEX books_is_rentable_idx ON books (is_rentable);
CREATE INDEX books_price_idx ON books (price);

-- +goose Down
DROP INDEX books_title_idx;
DROP INDEX books_author_idx;
DROP INDEX books_is_rentable_idx;
DROP INDEX books_price_idx;
DROP TABLE books;