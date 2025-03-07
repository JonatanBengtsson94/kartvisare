package repository

import "backend/internal/model"

type WmsRepository interface {
	GetAll() []model.Wms
}

type InMemoryWmsRepository struct {
	wms []model.Wms
}

func NewInMemoryWmsRepository() *InMemoryWmsRepository {
	return &InMemoryWmsRepository{
		wms: []model.Wms{
			{
				Name:        "Geoserver demo",
				Url:         "http://localhost:8001/geoserver/ows",
				Description: "Demo layer",
				Layers:      []string{"topp:states"},
			},
		},
	}
}

func (repo *InMemoryWmsRepository) GetAll() []model.Wms {
	return repo.wms
}
