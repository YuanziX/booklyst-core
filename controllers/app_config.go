package controllers

import "github.com/yuanzix/booklyst-core/internal/database"

type AppConfig struct {
	DB         *database.Queries
	SERVER_URL string
	PORT       string
	JWT_SECRET string
}
