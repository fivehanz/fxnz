package main

import (
	"net/http"
	"time"

	"github.com/labstack/echo/v4"
	"golang.org/x/net/http2"
)

func processRedirection(c echo.Context) error {
	ID := c.Param("ID")
	if ID == "bio" {
		c.Redirect(http.StatusMovedPermanently, "//withkoji.com/@fivehanz")
	}
	return c.String(http.StatusOK, ID)
}

func main() {
	e := echo.New()
	e.GET("/:ID", processRedirection)

	s := &http2.Server{
		MaxConcurrentStreams: 250,
		MaxReadFrameSize:     1048576,
		IdleTimeout:          10 * time.Second,
	}

	e.StartH2CServer(":8080", s)
}
