Culinary Social Media Website "CookBook"

Main Branch =>
  Repository submitted as a university project using rust PoemOpenAPI, SvelteKit, DUFS and MySQL
  To run the dev version:
  ```docker compose up```
  To run prod version: (optimised builds and https)
  ```docker compose -f docker-compose.yml -f docker-compose.prod.yml up```

Golang branch =>
  Ongoing rewrite to experiment with Golang, templ and htmx
  Uses pocketbase for database and file storage
  To setup dev version:
  ```npm install```
  Then
  ```make```
  Or for live reload with air
  ```air```
  Or to just build
  ```make build```
