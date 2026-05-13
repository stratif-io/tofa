.PHONY: install build preview docker-build docker-run docker-stop og posters check test e2e lhci clean

install:
	npm ci

build:
	npm run build

preview: build
	npm run preview

# Build the production Docker image locally
docker-build:
	docker build -t tofa-landing:dev .

# Run the image locally on port 8080
docker-run: docker-build
	-docker rm -f tofa-landing-dev 2>/dev/null
	docker run -d --name tofa-landing-dev -p 8080:80 tofa-landing:dev
	@echo "→ http://localhost:8080"

docker-stop:
	-docker rm -f tofa-landing-dev 2>/dev/null

og:
	npm run og

posters:
	npm run posters

check:
	npm run check

test:
	npm test

e2e:
	npm run e2e

lhci: build
	npm run lhci

clean:
	rm -rf dist .astro node_modules
