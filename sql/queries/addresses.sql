-- name: CreateAddress :one
INSERT INTO addresses (user_id, street, city, state, zip_code, country)
VALUES (?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: GetAddressByID :one
SELECT * FROM addresses
WHERE id = ?;

-- name: GetAddressesByUserID :many
SELECT * FROM addresses
WHERE user_id = ?;
