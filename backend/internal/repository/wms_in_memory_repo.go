package repository

import (
	"backend/internal/model"
	"sync"
)

type InMemoryWmsRepository struct {
	mu  sync.RWMutex
	wms map[int]model.Wms
}

func NewInMemoryWmsRepository() *InMemoryWmsRepository {
	return &InMemoryWmsRepository{
		wms: map[int]model.Wms{
			1: {
				ID:          1,
				Name:        "Geoserver demo",
				Url:         "http://localhost:8001/geoserver/ows",
				Description: "Demo layer",
				Layers:      []string{"topp:states"},
			},
		},
	}
}

func (r *InMemoryWmsRepository) GetAll() []model.Wms {
	r.mu.RLock()
	defer r.mu.RUnlock()

	wmsList := make([]model.Wms, 0, len(r.wms))
	for _, wms := range r.wms {
		wmsList = append(wmsList, wms)
	}
	return wmsList
}

func (r *InMemoryWmsRepository) GetById(id int) *model.Wms {
	r.mu.RLock()
	defer r.mu.RUnlock()

	wms, exists := r.wms[id]
	if !exists {
		return nil
	}
	return &wms
}
