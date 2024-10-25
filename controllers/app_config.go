package controllers

import (
	"database/sql"

	"github.com/yuanzix/booklyst-core/internal/database"
)

type AppConfig struct {
	Query      *database.Queries
	DB         *sql.DB
	SERVER_URL string
	PORT       string
	JWT_SECRET string
}
