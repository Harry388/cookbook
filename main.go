package main

import (
	"cookbook/handlers"
	"log"
	"os"

	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/core"
)

func main() {
    app := pocketbase.New()

    app.OnBeforeServe().Add(func(e *core.ServeEvent) error {
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
