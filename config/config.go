/*
* 	config
*	author: @fivehanz
*	updated: 20/04/2022
 */
package config

import (
	"github.com/golobby/config/v3"
	"github.com/golobby/config/v3/pkg/feeder"
)

type Configuration struct {
	App struct {
		Name string `env:"APP_NAME"`
		Port int    `env:"APP_PORT"`
	}
	DB struct {
		User         string `env:"DB_USER"`
		Password     string `env:"DB_PASSWORD"`
		Host         string `env:"DB_HOST"`
		Port         int    `env:"DB_PORT"`
		DatabaseName string `env:"DB_NAME"`
		Timezone     string `env:"DB_TIMEZONE"`
	}
}

func GetConfig() (Configuration, error) {
	myConfig := Configuration{}

	DotEnvFeeder := feeder.DotEnv{Path: ".env"}

	cfg := config.New()

	cfg.AddFeeder(DotEnvFeeder)
	cfg.AddStruct(&myConfig)
	err := cfg.Feed()

	return myConfig, err
}
