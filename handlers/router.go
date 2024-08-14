package handlers

import (
	"cookbook/middlewares"
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
    e.GET("/*", nilPage)
    e.GET("/favicon.ico", empty)

    // Auth
    e.GET("/login", h.loginPage)
    e.POST("/login", h.login)
    e.GET("/createaccount", h.createAccountPage)
    e.POST("/createaccount", h.createAccount)
    e.POST("/logout", h.logout)

    // User
    e.GET("/settings", h.settingsPage, middlewares.IsLoggedIn)
    e.GET("/profile", h.profilePage, middlewares.IsLoggedIn)
    e.POST("/profile", h.updateProfile, middlewares.IsLoggedIn)
    e.POST("/avatar", h.updateUserAvatar, middlewares.IsLoggedIn)
    e.GET("/user/:username", h.userPage)
    user := e.Group("/user/:username")
    user.GET("/avatar", h.userAvatar)
}

func nilPage(c echo.Context) error {
    c.Response().Header().Add("HX-Location", "/")
    return c.Redirect(301, "/")
}

func (h *handler) homePage(c echo.Context) error {
    t := home.HomePage()
    return templates.Render(t, c)
}

func empty(c echo.Context) error {
    return nil
}
