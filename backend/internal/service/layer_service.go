package service

import (
	"backend/internal/model"
	"backend/internal/repository"
)

type LayerService struct {
	repo repository.LayerRepository
}

func NewLayerService(repo repository.LayerRepository) *LayerService {
	return &LayerService{repo: repo}
}

func (s *LayerService) GetLayers() []model.Layer {
	return s.repo.GetAllLayers()
}
