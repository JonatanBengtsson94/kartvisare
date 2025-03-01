package main

import (
	"backend/internal/controller"
	"backend/internal/repository"
	"backend/internal/service"
	"net/http"
)

func main() {
	// Layers
	layerRepository := repository.NewInMemoryLayerRepository()
	layerService := service.NewLayerService(layerRepository)
	layerController := controller.NewLayerController(layerService)

	mux := http.NewServeMux()

	mux.HandleFunc("/layers", layerController.GetAllLayersHandler)

	http.ListenAndServe(":8080", mux)
}
