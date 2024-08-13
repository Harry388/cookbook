package handlers

import (
	"cookbook/templates"
	"cookbook/templates/settings"

	"github.com/labstack/echo/v5"
)

func (h *handler) settingsPage(c echo.Context) error {
    t := settings.SettingsPage()
    return templates.Render(t, c)
}
