-- +goose Up
ALTER TABLE order_items
RENAME COLUMN price TO price_per_item;

-- +goose Down
ALTER TABLE order_items
RENAME COLUMN price_per_item TO price;
