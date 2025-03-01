package controller

import (
	"backend/internal/service"
	"encoding/json"
	"net/http"
)

type LayerController struct {
	service *service.LayerService
}

func NewLayerController(service *service.LayerService) *LayerController {
	return &LayerController{service: service}
}

func (c *LayerController) GetAllLayersHandler(w http.ResponseWriter, r *http.Request) {
	layers := c.service.GetLayers()
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(layers)
}
