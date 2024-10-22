-- +goose Up
ALTER TABLE books ADD COLUMN img_url TEXT;

-- +goose Down
ALTER TABLE books DROP COLUMN img_url;