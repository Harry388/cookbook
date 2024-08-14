package handlers

import (
	"cookbook/templates"
	"cookbook/templates/user"
	"cookbook/templates/util"
	"net/http"

	"github.com/labstack/echo/v5"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/forms"
	"github.com/pocketbase/pocketbase/models"
)

type updateProfile struct {
    Username string `form:"username"`
    Name string `form:"name"`
    Bio *string `form:"bio"`
}

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

func (h *handler) updateUserAvatar(c echo.Context) error {
    record, _ := c.Get(apis.ContextAuthRecordKey).(*models.Record)

    form := forms.NewRecordUpsert(h.app, record)
    form.LoadRequest(c.Request(), "")

    if err := form.Submit(); err != nil {
        return err
    }

    return nil
}

func (h *handler) updateProfile(c echo.Context) error {
    record, _ := c.Get(apis.ContextAuthRecordKey).(*models.Record)

    var update updateProfile
    if err := c.Bind(&update); err != nil {
        c.Response().Status = 400;
        t := util.ErrAlert("Invalid Value")
        return templates.Render(t, c)
    }

    if update.Bio != nil {
        record.Set("bio", *update.Bio)
    }

    if update.Name != "" {
        record.Set("name", update.Name)
    }

    if update.Username != "" {
        record.SetUsername(update.Username)
    }

    if err := h.app.Dao().SaveRecord(record); err != nil {
        if update.Username != "" {
            c.Response().Status = 400;
            t := util.ErrAlert("Username is taken")
            return templates.Render(t, c)
        }
        c.Response().Status = 400;
        t := util.ErrAlert("Invalid Value")
        return templates.Render(t, c)
    }

    return nil
}
