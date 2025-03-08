package service

import (
	"backend/internal/model"
	"backend/internal/repository"
)

type UserService struct {
	repo repository.UserRepository
}

func NewUserService(repo repository.UserRepository) *UserService {
	return &UserService{repo: repo}
}

func (s *UserService) GetAll() ([]model.User, error) {
	return s.repo.GetAllUsers()
}

func (s *UserService) GetById(id int) (*model.User, error) {
	return s.repo.GetUserById(id)
}
