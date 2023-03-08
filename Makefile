.PHONY: build-user-development
build-user-development:
	@if [ ! -f users/docker/development/.env ]; then \
		cp users/.env.example users/docker/development/.env; \
	fi
	docker compose -f users/docker/development/docker-compose.yaml build

.PHONY: start-user-development
start-user-development:
	docker compose -f users/docker/development/docker-compose.yaml up -d

.PHONY: stop-user-development
stop-user-development:
	docker compose -f users/docker/development/docker-compose.yaml down

.PHONY: build-user-production
build-user-production:
	@if [ ! -f users/docker/production/.env ]; then \
		cp users/.env.example users/docker/production/.env; \
	fi
	docker compose -f docker/production/docker-compose.yaml build --no-cache

.PHONY: start-user-production
start-user-production:
	docker compose -f docker/production/docker-compose.yaml up -d

.PHONY: stop-user-production
stop-user-production:
	docker compose -f docker/production/docker-compose.yaml down
