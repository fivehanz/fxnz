/*
* 	routes for API endpoints
*	author: @fivehanz
*	updated: 20/04/2022
 */

package routes

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func API(app *echo.Echo) {
	app.GET("/api/:endpoint", func(c echo.Context) error {
		return c.JSON(http.StatusOK, []string{"sample1", "sample2", c.Param("endpoint")})
	})
}
