// Code generated by sqlc. DO NOT EDIT.
// versions:
//   sqlc v1.27.0

package database

import (
	"database/sql"
	"time"
)

type Address struct {
	ID        int64
	UserID    int64
	Street    sql.NullString
	City      sql.NullString
	State     sql.NullString
	ZipCode   sql.NullString
	Country   sql.NullString
	CreatedAt sql.NullTime
	UpdatedAt sql.NullTime
}

type Book struct {
	ID                int64
	Title             string
	Author            sql.NullString
	Description       sql.NullString
	Price             float64
	AvailableStock    sql.NullInt64
	IsRentable        sql.NullBool
	RentalPricePerDay sql.NullFloat64
	CreatedAt         sql.NullTime
	UpdatedAt         sql.NullTime
}

type CartItem struct {
	ID        int64
	UserID    int64
	BookID    int64
	Quantity  sql.NullInt64
	CreatedAt sql.NullTime
	UpdatedAt sql.NullTime
}

type Order struct {
	ID         int64
	UserID     int64
	TotalPrice float64
	Status     string
	CreatedAt  sql.NullTime
	UpdatedAt  sql.NullTime
}

type OrderItem struct {
	ID        int64
	OrderID   int64
	BookID    int64
	Quantity  sql.NullInt64
	Price     float64
	CreatedAt sql.NullTime
	UpdatedAt sql.NullTime
}

type Payment struct {
	ID            int64
	UserID        int64
	OrderID       int64
	Amount        float64
	PaymentMethod string
	PaymentStatus string
	CreatedAt     sql.NullTime
}

type Rental struct {
	ID              int64
	UserID          int64
	BookID          int64
	RentalStartDate time.Time
	RentalEndDate   time.Time
	TotalRentalCost float64
	Status          sql.NullString
	CreatedAt       sql.NullTime
	UpdatedAt       sql.NullTime
}

type Review struct {
	ID        int64
	BookID    int64
	UserID    int64
	Rating    sql.NullInt64
	Comment   sql.NullString
	CreatedAt sql.NullTime
	UpdatedAt sql.NullTime
}

type User struct {
	ID           int64
	FirstName    string
	LastName     string
	Email        string
	PasswordHash string
	Role         string
	CreatedAt    sql.NullTime
	UpdatedAt    sql.NullTime
}

type Wishlist struct {
	ID        int64
	UserID    int64
	BookID    int64
	CreatedAt sql.NullTime
}
