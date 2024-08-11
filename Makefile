default: run

build:
	@tailwindcss -i ./input.css -o ./pb_public/tailwind.css
	@templ generate
	@go build -o bin/app

run: build
	@./bin/app serve

clean:
	@rm -r bin
	@rm -rf **/*_templ.go
	@rm ./pb_public/tailwind.css
