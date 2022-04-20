/*
* 	routes
*	author: @fivehanz
*	updated: 20/04/2022
 */

package routes

import (
	"fmt"
	"net/http"

	"github.com/fivehanz/fxnz/config"
	"github.com/fivehanz/fxnz/links"
	"github.com/labstack/echo/v4"
)

func Routes(app *echo.Echo) {
	cfg, _ := config.GetConfig()
	// Home Route
	app.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, fmt.Sprintf("Hi! This is %v", cfg.App.Name))
	})

	// Route to redirect the respective URL
	app.GET("/:slug", links.ProcessSlug)
}
