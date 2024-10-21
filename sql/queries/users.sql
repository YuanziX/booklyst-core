-- name: CreateUser :one
INSERT INTO users (first_name, last_name, email, password_hash, role)
VALUES (?, ?, ?, ?, ?)
RETURNING id, first_name, last_name, email, role, created_at, updated_at;

-- name: GetAllUsers :many
SELECT id, first_name, last_name, email, role, created_at, updated_at
FROM users;

-- name: GetUserByEmail :one
SELECT id, first_name, last_name, email, role, created_at, updated_at
FROM users
WHERE email = ?;

-- name: GetUserIdRoleAndPasswordByEmail :one
SELECT id, role, password_hash
FROM users
WHERE email = ?;

-- name: DeleteUserByID :exec
DELETE FROM users
WHERE id = ?;
