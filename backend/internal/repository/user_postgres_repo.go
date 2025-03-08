package repository

import (
	"backend/internal/model"
	"database/sql"
	"fmt"
)

type PostgresUserRepository struct {
	db *sql.DB
}

func NewPostGresUserRepository(db *sql.DB) *PostgresUserRepository {
	return &PostgresUserRepository{db: db}
}

func (r *PostgresUserRepository) GetAllUsers() ([]model.User, error) {
	query := `SELECT user_id, username FROM users`

	rows, err := r.db.Query(query)
	if err != nil {
		return nil, fmt.Errorf("failed to execute query: %w", err)
	}
	defer rows.Close()

	var users []model.User
	for rows.Next() {
		var user model.User
		err := rows.Scan(&user.ID, &user.Username)
		if err != nil {
			return nil, fmt.Errorf("failed to scan user: %w", err)
		}
		users = append(users, user)
	}

	err = rows.Err()
	if err != nil {
		return nil, fmt.Errorf("failed to iterate over rows: %w", err)
	}

	return users, nil
}

func (r *PostgresUserRepository) GetUserById(userId int) (*model.User, error) {
	return nil, nil
}

func (r *PostgresUserRepository) GetAllUserGroups() ([]model.UserGroup, error) {
	return nil, nil
}

func (r *PostgresUserRepository) GetUserGroupsByUserId(userId int) ([]model.UserGroup, error) {
	return nil, nil
}

func (r *PostgresUserRepository) CreateUser(user model.User) (*model.User, error) {
	return nil, nil
}

func (r *PostgresUserRepository) CreateUserGroup(group model.UserGroup) (*model.UserGroup, error) {
	return nil, nil
}

func (r *PostgresUserRepository) AddUserToGroup(userId int, groupId int) error {
	return nil
}
