default: run

run:
	@tailwindcss -i ./input.css -o ./pb_public/tailwind.css
	@templ generate
	@go run main.go serve
