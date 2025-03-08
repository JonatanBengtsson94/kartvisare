package main

import (
	"backend/internal/controller"
	"backend/internal/repository"
	"backend/internal/service"
	"fmt"
	"net/http"
)

func main() {
	wmsRepository := repository.NewInMemoryWmsRepository()
	wmsService := service.NewWmsService(wmsRepository)
	wmsController := controller.NewWmsController(wmsService)

	mux := http.NewServeMux()

	mux.HandleFunc("/wms", wmsController.GetAllWmsHandler)
	mux.HandleFunc("/wms/", wmsController.GetWmsByIdHandler)

	fmt.Println("Server running on port 8080")
	http.ListenAndServe(":8080", mux)
}
