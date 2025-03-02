package repository

import "backend/internal/model"

type LayerRepository interface {
	GetAllLayers() []model.Wms
}

type InMemoryLayerRepository struct {
	layers []model.Wms
}

func NewInMemoryLayerRepository() *InMemoryLayerRepository {
	return &InMemoryLayerRepository{
		layers: []model.Wms{
			{
				Name: "Geoserver demo",
				Url:  "http://localhost:8001/geoserver/ows",
				Params: model.WmsParams{
					Layers:  "topp:states",
					Version: "1.1.1",
					Format:  "image/png",
					SRS:     "EPSG:4326",
				},
			},
		},
	}
}

func (repo *InMemoryLayerRepository) GetAllLayers() []model.Wms {
	return repo.layers
}
