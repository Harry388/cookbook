default: run

build:
	@npx tailwindcss -i ./input.css -o ./pb_public/tailwind.css
	@templ generate
	@go build -o cookbook

run: build
	@./cookbook serve

clean:
	@rm cookbook
	@rm -rf **/*_templ.go
	@rm ./pb_public/tailwind.css
