package main

import (
	"fmt"
	"net/http"
	"time"

	"github.com/labstack/echo/v4"
	"golang.org/x/net/http2"
)

func processRedirection(c echo.Context) error {
	slug := c.Param("slug")
	return c.String(http.StatusOK, slug)
}

func main() {
	port := 8080

	e := echo.New()

	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusNotFound, http.StatusText(http.StatusNotFound))
	})

	e.GET("/:slug", processRedirection)

	e.GET("/api/:endpoint", func(c echo.Context) error {
		return c.JSON(http.StatusOK, []string{"sample1", "sample2", c.Param("endpoint")})
	})

	s := &http2.Server{
		MaxConcurrentStreams: 250,
		MaxReadFrameSize:     1048576,
		IdleTimeout:          10 * time.Second,
	}

	e.Logger.Fatal(e.StartH2CServer(fmt.Sprintf("127.0.0.1:%v", port), s))

}
