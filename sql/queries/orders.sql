-- name: CreateOrder :one
INSERT INTO orders (user_id, address_id, total_price)
VALUES (?, ?, ?)
RETURNING *;

-- name: CreateOrderItems :exec
INSERT INTO order_items (order_id, book_id, quantity, price_per_item)
VALUES (?, ?, ?, ?);

-- name: GetOrderByUserId :many
SELECT * FROM orders WHERE user_id = ?;