-- name: GetCartItemsByUserID :many
SELECT c.book_id, c.quantity, b.title, b.price
FROM cart_items c
JOIN books b
ON c.book_id = b.id
WHERE user_id = ?;

-- name: AddItemToCart :exec
INSERT INTO cart_items (user_id, book_id, quantity, created_at, updated_at)
VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT(user_id, book_id)
DO UPDATE SET
    quantity = excluded.quantity,
    updated_at = CURRENT_TIMESTAMP;

-- name: RemoveItemFromCart :exec
DELETE FROM cart_items
WHERE user_id = ? AND book_id = ?;

-- name: ClearCart :exec
DELETE FROM cart_items
WHERE user_id = ?;