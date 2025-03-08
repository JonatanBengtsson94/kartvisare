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
	wms, error := c.service.GetAll()
	if error != nil {
		http.Error(w, "Failed to fetch WMS", http.StatusInternalServerError)
	}
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
		return
	}

	wms, error := c.service.GetById(id)
	if error != nil {
		http.Error(w, "Failed to fetch WMS", http.StatusInternalServerError)
	}
	if wms == nil {
		http.Error(w, "WMS not found", http.StatusNotFound)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(wms)
}
