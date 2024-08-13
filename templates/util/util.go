package util

import (
	"context"

	"github.com/a-h/templ"
)

func Url(path string) string {
    return string(templ.URL(path))
}

func IsLoggedIn(ctx context.Context) bool {
    return ctx.Value("auth") != nil
}
