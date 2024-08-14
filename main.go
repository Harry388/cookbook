package main

import (
	"cookbook/handlers"
	"cookbook/middlewares"
	"log"
	"os"

	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/core"
    "github.com/joho/godotenv"
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

    if err := app.Start(); err != nil {
        log.Fatal(err)
    }
}
