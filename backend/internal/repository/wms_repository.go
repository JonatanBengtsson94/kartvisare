package repository

import (
	"backend/internal/model"
)

type WmsRepository interface {
	GetAll() ([]model.Wms, error)
	GetById(int) (*model.Wms, error)
}
