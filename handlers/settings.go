package handlers

import (
	"cookbook/templates"
	"cookbook/templates/settings"

	"github.com/labstack/echo/v5"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/models"
)

func (h *handler) settingsPage(c echo.Context) error {
    user := c.Get(apis.ContextAuthRecordKey).(*models.Record)
    t := settings.SettingsPage(user)
    return templates.Render(t, c)
}
