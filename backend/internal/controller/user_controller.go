package controller

import (
	"backend/internal/service"
	"encoding/json"
	"net/http"
)

type UserController struct {
	service *service.UserService
}

func NewUserController(service *service.UserService) *UserController {
	return &UserController{service: service}
}

func (c *UserController) GetAllUsersHandler(w http.ResponseWriter, r *http.Request) {
	user, error := c.service.GetAll()
	if error != nil {
		http.Error(w, "Failed to fetch users", http.StatusInternalServerError)
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(user)
}
