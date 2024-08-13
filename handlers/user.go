package handlers

import (
	"cookbook/templates"
	"cookbook/templates/user"
	"net/http"

	"github.com/labstack/echo/v5"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/models"
)

func (h *handler) profilePage(c echo.Context) error {
    record, _ := c.Get(apis.ContextAuthRecordKey).(*models.Record)
    t := user.UserPage(record)
    return templates.Render(t, c)
}

func (h *handler) userAvatar(c echo.Context) error {
    username := c.PathParam("username")
    if username == "" {
        return echo.NewHTTPError(http.StatusBadRequest)
    }

    record, err := h.app.Dao().FindAuthRecordByUsername("users", username)
    if err != nil {
        return echo.NewHTTPError(http.StatusNotFound)
    }

    key := record.BaseFilesPath() + "/" + record.GetString("avatar")

    fsys, _ := h.app.NewFilesystem()
    defer fsys.Close()

    blob, _ := fsys.GetFile(key)
    defer blob.Close()

    _, err = blob.WriteTo(c.Response())
    return err
}
