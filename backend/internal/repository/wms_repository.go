package repository

import "backend/internal/model"

type WmsRepository interface {
	GetAll() []model.Wms
	GetById(int) *model.Wms
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

func (r *InMemoryWmsRepository) GetAll() []model.Wms {
	return r.wms
}

func (r *InMemoryWmsRepository) GetById(id int) *model.Wms {
	for _, wms := range r.wms {
		if wms.ID == id {
			return &wms
		}
	}
	return nil
}
