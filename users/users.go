/*
* 	users: CRUD and utils
*	author: @fivehanz
*	updated: 20/04/2022
 */
package users

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func CreateUser(c echo.Context) error {
	return c.String(http.StatusOK, "")
}

func UpdateUser(c echo.Context) error {
	return c.String(http.StatusOK, "")
}

func DeletedUser(c echo.Context) error {
	return c.String(http.StatusOK, "")
}

func ReadUser(c echo.Context) error {
	return c.String(http.StatusOK, "")
}
