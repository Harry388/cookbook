package handlers

import (
	"cookbook/templates"
	"cookbook/templates/user"

	"github.com/labstack/echo/v5"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/models"
)

func (h *handler) profilePage(c echo.Context) error {
    record, ok := c.Get(apis.ContextAuthRecordKey).(*models.Record)
    if record == nil || !ok {
        return c.Redirect(301, "/")
    }
    t := user.UserPage(record)
    return templates.Render(t, c)
}
