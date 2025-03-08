package repository

import (
	"backend/internal/model"
	"database/sql"
)

type PostgresUserRepository struct {
	db *sql.DB
}

func NewPostGresUserRepository(db *sql.DB) *PostgresUserRepository {
	return &PostgresUserRepository{db: db}
}

func (r *PostgresUserRepository) GetAllUsers() ([]model.User, error) {
	return nil, nil
}
