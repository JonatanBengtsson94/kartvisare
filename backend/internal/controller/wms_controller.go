package controller

import (
	"backend/internal/service"
	"encoding/json"
	"net/http"
	"strconv"
	"strings"
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

func (c *WmsController) GetWmsByIdHandler(w http.ResponseWriter, r *http.Request) {
	pathParts := strings.Split(r.URL.Path, "/")
	if len(pathParts) < 3 {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	idStr := pathParts[2]
	id, err := strconv.Atoi(idStr)
	if err != nil {
		http.Error(w, "Invalid ID", http.StatusBadRequest)
	}

	wms := c.service.GetById(id)
	if wms == nil {
		http.Error(w, "WMS not found", http.StatusNotFound)
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(wms)
}
