package handlers

import (
	"cookbook/templates"
	"cookbook/templates/home"

	"github.com/labstack/echo/v5"
	"github.com/pocketbase/pocketbase"
)

type handler struct {
    app *pocketbase.PocketBase
}

func Handle(e *echo.Echo, app *pocketbase.PocketBase) {
    h := handler { app }
    e.GET("/", h.homePage)
    e.GET("/*", h.nilPage)

    // Auth
    e.GET("/login", h.loginPage)
    e.POST("/login", h.login)
    e.GET("/createaccount", h.createAccountPage)
    e.POST("/createaccount", h.createAccount)
    e.POST("/logout", h.logout)

    // User
    e.GET("/profile", h.profilePage)
}

func (h *handler) nilPage(c echo.Context) error {
    c.Response().Header().Add("HX-Location", "/")
    return c.Redirect(301, "/")
}

func (h *handler) homePage(c echo.Context) error {
    t := home.HomePage()
    return templates.Render(t, c)
}
