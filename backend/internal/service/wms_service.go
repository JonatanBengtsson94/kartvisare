package service

import (
	"backend/internal/model"
	"backend/internal/repository"
)

type WmsService struct {
	repo repository.WmsRepository
}

func NewWmsService(repo repository.WmsRepository) *WmsService {
	return &WmsService{repo: repo}
}

func (s *WmsService) GetAll() ([]model.Wms, error) {
	return s.repo.GetAll()
}

func (s *WmsService) GetById(id int) (*model.Wms, error) {
	return s.repo.GetById(id)
}
