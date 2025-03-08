package repository

import (
	"backend/internal/model"
	"database/sql"
	"fmt"
	"os"
)

type PostgresWmsRepository struct {
	db *sql.DB
}

func NewPostGresWmsRepository(db *sql.DB) (*PostgresWmsRepository, error) {
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

	return &PostgresWmsRepository{db: db}, nil
}

func (r *PostgresWmsRepository) GetAll() ([]model.Wms, error) {
	return nil, nil
}

func (r *PostgresWmsRepository) GetById(id int) (*model.Wms, error) {
	return nil, nil
}
