package repository

import (
	"backend/internal/model"
	"database/sql"
)

type PostgresWmsRepository struct {
	db *sql.DB
}

func NewPostGresWmsRepository(db *sql.DB) *PostgresWmsRepository {
	return &PostgresWmsRepository{db: db}
}

func (r *PostgresWmsRepository) GetAll() ([]model.Wms, error) {
	return nil, nil
}

func (r *PostgresWmsRepository) GetById(id int) (*model.Wms, error) {
	return nil, nil
}
