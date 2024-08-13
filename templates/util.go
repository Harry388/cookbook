package templates

import (
	"context"

	"github.com/a-h/templ"
	"github.com/labstack/echo/v5"
	"github.com/pocketbase/pocketbase/apis"
)

func Render(t templ.Component, c echo.Context) error {
    ctx := context.WithValue(c.Request().Context(), "auth", c.Get(apis.ContextAuthRecordKey))
    return t.Render(ctx, c.Response())
}
