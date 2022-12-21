.PHONY: build-user-development
build-user-development:
	@if [ ! -f users/docker/development/.env ]; then \
		cp users/.env users/docker/development/.env; \
	fi
	docker compose -f users/docker/development/docker-compose.yaml build

.PHONY: start-user-development
start-user-development:
	docker compose -f users/docker/development/docker-compose.yaml up -d

.PHONY: stop-user-development
stop-user-development: ## Stop the development docker container.
	docker compose -f users/docker/development/docker-compose.yaml down

.PHONY: build-staging
build-staging: ## Build the staging docker image.
	docker compose -f docker/staging/docker-compose.yml build --no-cache

.PHONY: start-staging
start-staging: ## Start the staging docker container.
	docker compose -f docker/staging/docker-compose.yml up -d

.PHONY: stop-staging
stop-staging: ## Stop the staging docker container.
	docker compose -f docker/staging/docker-compose.yml down

.PHONY: build-production
build-production: ## Build the production docker image.
	docker compose -f docker/production/docker-compose.yml build --no-cache

.PHONY: start-production
start-production: ## Start the production docker container.
	docker compose -f docker/production/docker-compose.yml up -d

.PHONY: stop-production
stop-production: ## Stop the production docker container.
	docker compose -f docker/production/docker-compose.yml down
