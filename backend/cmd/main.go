package main

import (
	"backend/internal/controller"
	"backend/internal/repository"
	"backend/internal/service"
	"database/sql"
	"fmt"
	"net/http"
	"os"
)

func main() {
	/*
		db, err := initPg()
		if err != nil {
			os.Exit(1)
		}

		wmsRepository := repository.NewPostGresWmsRepository(db)
	*/
	wmsRepository := repository.NewInMemoryWmsRepository()
	wmsService := service.NewWmsService(wmsRepository)
	wmsController := controller.NewWmsController(wmsService)

	mux := http.NewServeMux()

	mux.HandleFunc("/wms", wmsController.GetAllWmsHandler)
	mux.HandleFunc("/wms/", wmsController.GetWmsByIdHandler)

	fmt.Println("Server running on port 8080")
	http.ListenAndServe(":8080", mux)
}

func initPg() (*sql.DB, error) {
	dbHost := os.Getenv("DB_HOST")
	dbPort := os.Getenv("DB_PORT")
	dbUser := os.Getenv("DB_USER")
	dbPassword := os.Getenv("DB_PASSWORD")
	dbName := os.Getenv("DB_NAME")

	connectionString := fmt.Sprintf("host=%s port=%s user=%s password=%s dbname=%s sslmode=disable",
		dbHost, dbPort, dbUser, dbPassword, dbName)

	db, err := sql.Open("postgres", connectionString)
	if err != nil {
		return nil, fmt.Errorf("Failed to connect to the database: %w", err)
	}
	return db, nil
}
