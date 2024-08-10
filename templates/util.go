package templates

import (
	"github.com/a-h/templ"
	"github.com/labstack/echo/v5"
)

func Render(t templ.Component, c echo.Context) error {
    return t.Render(c.Request().Context(), c.Response())
}
