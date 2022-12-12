/*
* 	main server
*	author: @fivehanz
*	updated: 12/12/2022
 */
package main

import (
	//	"fmt"
	"time"

	//	"github.com/fivehanz/fxnz/config"
	"github.com/fivehanz/fxnz/routes"
	"github.com/labstack/echo/v4"
	"golang.org/x/net/http2"
)

func main() {
	// TODO: handle config errors
	//	cfg, _ := config.GetConfig()

	// create a new instance of echo
	app := echo.New()

	// import Routes from routes package
	routes.Routes(app)
	routes.API(app)

	// setup http2 server
	s := &http2.Server{
		MaxConcurrentStreams: 250,
		MaxReadFrameSize:     1048576,
		IdleTimeout:          10 * time.Second,
	}

	// TODO: handle empty config with default values
	// default port
	//Port := 8080

	// Start HTTP2 Server with Fatal Logger
	//app.Logger.Fatal(app.StartH2CServer(fmt.Sprintf("0.0.0.0:%v", cfg.App.Port), s))
	app.Logger.Fatal(app.StartH2CServer("0.0.0.0:8080", s))
}
