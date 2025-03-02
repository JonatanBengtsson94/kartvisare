package repository

import "backend/internal/model"

type LayerRepository interface {
	GetAllLayers() []model.Layer
}

type InMemoryLayerRepository struct {
	layers []model.Layer
}

func NewInMemoryLayerRepository() *InMemoryLayerRepository {
	return &InMemoryLayerRepository{
		layers: []model.Layer{
			{Name: "Geoserver demo", Url: "http://localhost:8001/geoserver/ows"},
		},
	}
}

func (repo *InMemoryLayerRepository) GetAllLayers() []model.Layer {
	return repo.layers
}
