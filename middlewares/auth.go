package middlewares

import (
	"github.com/labstack/echo/v5"
	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/models"
)

func AuthMiddleware(app *pocketbase.PocketBase) echo.MiddlewareFunc {
    return func(next echo.HandlerFunc) echo.HandlerFunc {
        return func(c echo.Context) error {
            tokenCookie, err := c.Request().Cookie("Authorization")
            if err != nil || tokenCookie.Value == "" {
                return next(c) // no token cookie
            }

            token := tokenCookie.Value

            record, err := app.Dao().FindAuthRecordByToken(
                token,
                app.Settings().RecordAuthToken.Secret,
            )
            if err == nil && record != nil {
                // "authenticate" the app user
                c.Set(apis.ContextAuthRecordKey, record)
            }

            return next(c)
        }
    }
}

func IsLoggedIn(next echo.HandlerFunc) echo.HandlerFunc {
    return func(c echo.Context) error {
        record, ok := c.Get(apis.ContextAuthRecordKey).(*models.Record)
        if record == nil || !ok {
            return c.Redirect(301, "/")
        }
        return next(c)
    }
}
