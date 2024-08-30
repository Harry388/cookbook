# Culinary Social Media Website "CookBook"

## Main Branch
Repository submitted as a university project using rust PoemOpenAPI, SvelteKit, DUFS and MySQL
To run the dev version:
```bash
docker compose up
```
To run prod version: (optimised builds and https)
```bash
docker compose -f docker-compose.yml -f docker-compose.prod.yml up
```

## Golang Branch
Ongoing rewrite to experiment with Golang, templ and htmx
Uses pocketbase for database and file storage
To setup dev version:
```bash
npm install
```
Then
```bash make
```
Or for live reload with air
```bash
air
```
Or to just build
```bash
make build
```
