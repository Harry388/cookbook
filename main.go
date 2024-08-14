package main

import (
	"cookbook/handlers"
	"cookbook/middlewares"
	"log"
	"os"

	"github.com/joho/godotenv"
	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/core"
	"github.com/pocketbase/pocketbase/plugins/migratecmd"

    _ "cookbook/migrations"
)

func main() {
    godotenv.Load()

    app := pocketbase.New()

    app.OnBeforeServe().Add(func(e *core.ServeEvent) error {
        e.Router.Use(middlewares.AuthMiddleware(app))
        // Serve Static
        e.Router.GET("/public/*", apis.StaticDirectoryHandler(os.DirFS("./pb_public"), false))
        // Serve App
        handlers.Handle(e.Router, app)
        return nil
    })

    migratecmd.MustRegister(app, app.RootCmd, migratecmd.Config{
        // enable auto creation of migration files when making collection changes in the Admin UI
        Automigrate: os.Getenv("dev") == "true",
    })

    if err := app.Start(); err != nil {
        log.Fatal(err)
    }
}
