default: run

run:
	@templ generate
	@go run main.go serve
