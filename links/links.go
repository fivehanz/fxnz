/*
* 	links: CRUD and utils
*	author: @fivehanz
*	updated: 20/04/2022
 */
package links

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func CreateLink(c echo.Context) error {
	return c.String(http.StatusOK, "")
}

func UpdateLink(c echo.Context) error {
	return c.String(http.StatusOK, "")
}

func DeletedLink(c echo.Context) error {
	return c.String(http.StatusOK, "")
}

func ReadLink(c echo.Context) error {
	return c.String(http.StatusOK, "")
}

func ProcessSlug(c echo.Context) error {
	slug := c.Param("slug")
	return c.String(http.StatusOK, slug)
}
