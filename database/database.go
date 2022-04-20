/*
* 	database
*	author: @fivehanz
*	updated: 20/04/2022
 */
package database

import (
	"fmt"

	"github.com/fivehanz/fxnz/config"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

func Connect() (*gorm.DB, error) {
	cfg, _ := config.GetConfig()

	// DSN := "host=localhost user=gorm password=gorm dbname=gorm port=9920 sslmode=disable TimeZone=Asia/Singapore"
	DSN := fmt.Sprintf("user=%v password=%v host=%v port=%v dbname=%v %v",
		cfg.DB.User,
		cfg.DB.Password,
		cfg.DB.Host,
		cfg.DB.Port,
		cfg.DB.DatabaseName,
		cfg.DB.Timezone,
	)

	db, err := gorm.Open(postgres.Open(DSN), &gorm.Config{})
	return db, err
}
