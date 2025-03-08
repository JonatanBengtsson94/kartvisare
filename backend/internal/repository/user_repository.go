package repository

import "backend/internal/model"

type UserRepository interface {
	GetAllUsers() ([]model.User, error)
	GetUserById(userId int) (*model.User, error)
	GetAllUserGroups() ([]model.UserGroup, error)
	GetUserGroupsByUserId(userId int) ([]model.UserGroup, error)
	CreateUser(user model.User) (*model.User, error)
	CreateUserGroup(group model.UserGroup) (*model.UserGroup, error)
	AddUserToGroup(userId int, groupId int) error
}
