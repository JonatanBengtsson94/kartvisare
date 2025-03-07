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

func (s *WmsService) GetById(id int) *model.Wms {
	return s.repo.GetById(id)
}
