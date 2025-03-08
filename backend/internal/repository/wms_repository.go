package repository

import (
	"backend/internal/model"
)

type WmsRepository interface {
	GetAll() []model.Wms
	GetById(int) *model.Wms
}
