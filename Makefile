.PHONY: user-development
user-development:
	docker compose -f users/docker/development/docker-compose.yaml down

	@if [ ! -f users/docker/development/.env ]; then \
		cp users/.env.example users/docker/development/.env; \
	fi
	docker compose -f users/docker/development/docker-compose.yaml build

	docker compose -f users/docker/development/docker-compose.yaml up -d

.PHONY: user-production
user-production:
	docker compose -f users/docker/production/docker-compose.yaml down

	@if [ ! -f users/docker/production/.env ]; then \
		cp users/.env.example users/docker/production/.env; \
	fi
	docker compose -f users/docker/production/docker-compose.yaml build --no-cache

	docker compose -f users/docker/production/docker-compose.yaml up -d

.PHONY: user-deploy
user-deploy:
	docker compose -f users/docker/deploy/docker-compose.yaml up -d

# Haproxy
.PHONY: haproxy
haproxy:
	docker compose -f docker/docker-compose-haproxy.yaml up -d