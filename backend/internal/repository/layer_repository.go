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
			{Name: "Topoweb", Url: "https://api.lantmateriet.se/open/topowebb-ccby/v1/wmts?request=GetCapabilities&version=1.0.0&service=wmts"},
		},
	}
}

func (repo *InMemoryLayerRepository) GetAllLayers() []model.Layer {
	return repo.layers
}
