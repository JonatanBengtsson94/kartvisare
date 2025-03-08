package repository

import (
	"backend/internal/model"
	"errors"
	"sync"
)

type InMemoryUserRepository struct {
	mu    sync.RWMutex
	users map[int]model.User
}

func NewInMemoryUserRepository() *InMemoryUserRepository {
	return &InMemoryUserRepository{
		users: map[int]model.User{
			1: {
				ID:       1,
				Username: "Testuser",
			},
		},
	}
}

func (r *InMemoryUserRepository) GetAll() ([]model.User, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	users := make([]model.User, 0, len(r.users))
	for _, user := range r.users {
		users = append(users, user)
	}
	return users, nil
}

func (r *InMemoryUserRepository) GetById(id int) (*model.User, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	user, exists := r.users[id]
	if !exists {
		return nil, errors.New("User not found")
	}
	return &user, nil
}
