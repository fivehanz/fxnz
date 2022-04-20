package models

import "gorm.io/gorm"

type Link struct {
	gorm.Model
	Slug string
	URL  string
	User User
}
