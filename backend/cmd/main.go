package main

import (
	"backend/internal/controller"
	"backend/internal/repository"
	"backend/internal/service"
	"net/http"
)

func main() {
	wmsRepository := repository.NewInMemoryWmsRepository()
	wmsService := service.NewWmsService(wmsRepository)
	wmsController := controller.NewWmsController(wmsService)

	mux := http.NewServeMux()

	mux.HandleFunc("/wms", wmsController.GetAllWmsHandler)
	mux.HandleFunc("/wms/", wmsController.GetWmsByIdHandler)

	http.ListenAndServe(":8080", mux)
}
