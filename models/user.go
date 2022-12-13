package models

import (
	"gorm.io/gorm"
)

type User struct {
	gorm.Model
	Name     string
	Username string
	Email    *string
	Admin    bool `gorm:"default:false"`
}
