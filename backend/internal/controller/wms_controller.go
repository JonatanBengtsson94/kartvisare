package controller

import (
	"backend/internal/service"
	"encoding/json"
	"net/http"
)

type WmsController struct {
	service *service.WmsService
}

func NewWmsController(service *service.WmsService) *WmsController {
	return &WmsController{service: service}
}

func (c *WmsController) GetAllWmsHandler(w http.ResponseWriter, r *http.Request) {
	wms := c.service.GetAll()
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(wms)
}
