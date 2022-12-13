/*
* 	routes
*	author: @fivehanz
*	updated: 12/12/2022
 */

package routes

import (
	//	"fmt"
	"net/http"

	//	"github.com/fivehanz/fxnz/config"
	"github.com/fivehanz/fxnz/links"
	"github.com/labstack/echo/v4"
)

func Routes(app *echo.Echo) {
	// Home Route
	app.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hey, if you can read this -- ArgoCD is working -- #4")
	})

	// Route to redirect the respective URL
	app.GET("/:slug", links.ProcessSlug)
}
