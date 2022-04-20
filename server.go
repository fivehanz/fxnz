/*
* 	main server
*	author: @fivehanz
*	updated: 20/04/2022
 */
package main

import (
	"fmt"
	"time"

	"github.com/fivehanz/fxnz/config"
	"github.com/fivehanz/fxnz/routes"
	"github.com/labstack/echo/v4"
	"golang.org/x/net/http2"
)

func main() {
	cfg, _ := config.GetConfig()

	// create a new instance of echo
	app := echo.New()

	// Import Routes from routes package
	routes.Routes(app)
	routes.API(app)

	// Start HTTP2 Server with Fatal Logger
	s := &http2.Server{
		MaxConcurrentStreams: 250,
		MaxReadFrameSize:     1048576,
		IdleTimeout:          10 * time.Second,
	}
	app.Logger.Fatal(app.StartH2CServer(fmt.Sprintf("0.0.0.0:%v", cfg.App.Port), s))

}
