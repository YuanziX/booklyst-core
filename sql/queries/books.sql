-- name: GetBooksByOffset :many
WITH book_count AS (
    SELECT COUNT(*) AS total_count FROM books
)
SELECT total_count, b.*
FROM book_count, (
    SELECT * FROM books
    ORDER BY id
    LIMIT ? OFFSET ?
) AS b;

-- name: GetBookByID :one
SELECT id, title, author, description, price, available_stock, is_rentable, rental_price_per_day, img_url
FROM books
WHERE id = ?;