package handlers

import (
	"cookbook/templates"
	"net/http"
	"time"

	"github.com/labstack/echo/v5"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/models"
)

type login struct {
    Username string `form:"username"`
    Password string `form:"password"`
}

type create struct {
    Username string `form:"username"`
    Name string `form:"name"`
    Password string `form:"password"`
}

func (h *handler) loginPage(c echo.Context) error {
    t := templates.LoginPage()
    return templates.Render(t, c)
}

func (h *handler) createAccountPage(c echo.Context) error {
    t := templates.CreatePage()
    return templates.Render(t, c)
}

func (h *handler) login(c echo.Context) error {
    var login login
    if err := c.Bind(&login); err != nil {
        return c.String(http.StatusBadRequest, "Incorrect Login Details")
    }

    record, err := h.app.Dao().FindFirstRecordByData("users", "username", login.Username)
    if err != nil || !record.ValidatePassword(login.Password) {
        t := templates.LoginForm(login.Username, login.Password, "Incorrect Login Details")
        return templates.Render(t, c)
    }


    c.Response().Header().Add("HX-Location", "/")
    return apis.RecordAuthResponse(h.app, c, record, nil, func(token string) error {
        cookie := &http.Cookie {
            Name: "Authorization",
            Value: token,
            Secure: true,
            SameSite: http.SameSiteStrictMode,
            HttpOnly: true,
        }
        c.SetCookie(cookie)
        return nil
    })
}

func (h *handler) createAccount(c echo.Context) error {
    var create create
    if err := c.Bind(&create); err != nil {
        return c.String(http.StatusBadRequest, "Bad Request")
    }

    users, err := h.app.Dao().FindCollectionByNameOrId("users")
    if err != nil {
        t := templates.CreateForm(create.Username, create.Name, create.Password, "Something went wrong")
        return templates.Render(t, c)
    }

    record := models.NewRecord(users)
    record.SetUsername(create.Username)
    record.SetPassword(create.Password)
    record.Set("name", create.Name)

    if err := h.app.Dao().SaveRecord(record); err != nil {
        t := templates.CreateForm(create.Username, create.Name, create.Password, "Something went wrong")
        return templates.Render(t, c)
    }

    c.Response().Header().Add("HX-Location", "/")
    return apis.RecordAuthResponse(h.app, c, record, nil, func(token string) error {
        cookie := &http.Cookie {
            Name: "Authorization",
            Value: token,
            Secure: true,
            SameSite: http.SameSiteStrictMode,
        }
        c.SetCookie(cookie)
        return nil
    })
}

func (h *handler) logout(c echo.Context) error {
    c.Response().Header().Add("HX-Location", "/login")
    cookie := &http.Cookie {
        Name: "Authorization",
        Value: "deleted",
        Expires: time.Date(1970, 1, 1, 0, 0, 0, 0, time.UTC),
    }
    c.SetCookie(cookie)
    return nil
}
