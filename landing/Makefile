.PHONY: install build preview deploy og posters check test e2e lhci clean

install:
	npm ci

build:
	npm run build

preview: build
	npm run preview

deploy: build
	bash scripts/deploy.sh

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
