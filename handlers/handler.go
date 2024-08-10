package handlers

import (
	"cookbook/templates"

	"github.com/labstack/echo/v5"
	"github.com/pocketbase/pocketbase"
)

type handler struct {
    app *pocketbase.PocketBase
}

func Handle(e *echo.Echo, app *pocketbase.PocketBase) {
    h := handler { app }
    e.GET("/hello/:name", h.hello)
}

func (h *handler) hello(c echo.Context) error {
    name := c.PathParam("name")
    t := templates.Home(name)
    return templates.Render(t, c)
}
