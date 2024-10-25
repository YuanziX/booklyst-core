package main

import (
	"database/sql"
	"log"
	"net/http"
	"os"

	"github.com/joho/godotenv"
	_ "github.com/mattn/go-sqlite3"
	"github.com/yuanzix/booklyst-core/controllers"
	"github.com/yuanzix/booklyst-core/internal/database"
	"github.com/yuanzix/booklyst-core/utils"
)

func main() {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	JWT_SECRET := os.Getenv("JWT_SECRET")
	if JWT_SECRET == "" {
		log.Fatal("JWT_SECRET is required")
	}

	PORT := os.Getenv("PORT")
	if PORT == "" {
		PORT = "3000"
	}

	SERVER_URL := os.Getenv("SERVER_URL")
	if SERVER_URL == "" {
		SERVER_URL = "http://localhost"
	}

	db, err := sql.Open("sqlite3", "./app.db")
	if err != nil {
		log.Fatal("Unable to connect to database:", err)
	}

	queries := database.New(db)

	appConfig := controllers.AppConfig{
		Query:      queries,
		DB:         db,
		SERVER_URL: SERVER_URL,
		PORT:       PORT,
		JWT_SECRET: JWT_SECRET,
	}

	router := InitializeRoutes(appConfig)
	server := &http.Server{
		Handler: utils.CORSHandler(router),
		Addr:    ":" + PORT,
	}

	log.Printf("Server starting on port %v.\n", PORT)
	err = server.ListenAndServe()
	if err != nil {
		log.Fatal(err)
	}
}
