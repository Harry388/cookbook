package handlers

import (
	"cookbook/templates"

	"github.com/labstack/echo/v5"
	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/models"
)

type handler struct {
    app *pocketbase.PocketBase
}

func Handle(e *echo.Echo, app *pocketbase.PocketBase) {
    h := handler { app }
    e.GET("/", h.homePage)

    // Auth
    e.GET("/login", h.loginPage)
    e.POST("/login", h.login)
    e.GET("/createaccount", h.createAccountPage)
    e.POST("/createaccount", h.createAccount)
    e.POST("/logout", h.logout)
}

func (h *handler) homePage(c echo.Context) error {
    name := "No User"
    record, _ := c.Get(apis.ContextAuthRecordKey).(*models.Record)
    if record != nil {
        name = record.GetString("username")
    }
    t := templates.Home(name)
    return templates.Render(t, c)
}
