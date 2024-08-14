package util

import (
	"context"
	"fmt"

	"github.com/a-h/templ"
)

func SUrl(format string, a ...any) string {
    return string(templ.URL(fmt.Sprintf(format, a...)))
}

func Url(format string, a ...any) templ.SafeURL {
    return templ.URL(fmt.Sprintf(format, a...))
}

func IsLoggedIn(ctx context.Context) bool {
    return ctx.Value("auth") != nil
}
