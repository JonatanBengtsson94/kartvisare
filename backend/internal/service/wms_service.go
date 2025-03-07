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

func (s *WmsService) GetAll() []model.Wms {
	return s.repo.GetAll()
}
