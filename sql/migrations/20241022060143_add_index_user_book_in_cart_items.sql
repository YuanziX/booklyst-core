-- +goose Up
CREATE UNIQUE INDEX idx_cart_items_user_book ON cart_items(user_id, book_id);

-- +goose Down
DROP INDEX idx_cart_items_user_book;