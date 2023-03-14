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
	docker compose -f users/docker/production/docker-compose.yaml build

	docker compose -f users/docker/production/docker-compose.yaml up -d

.PHONY: deploy
deploy:
	@if [ -z $$(docker network ls --filter name=haproxy_network -q) ]; then \
		docker network create haproxy_network; \
		echo "Network haproxy_network has been created successfully"; \
	else \
		echo "Network haproxy_network already exists"; \
	fi

	docker compose -f deploy/users/github/docker-compose.yaml down
	docker compose -f deploy/users/gitlab/docker-compose.yaml down
	docker compose -f deploy/web-assembly/docker-compose.yaml down
	docker compose -f deploy/haproxy/docker-compose.yaml down

	docker compose -f deploy/users/github/docker-compose.yaml build
	docker compose -f deploy/users/gitlab/docker-compose.yaml build
	docker compose -f deploy/web-assembly/docker-compose.yaml build
	docker compose -f deploy/haproxy/docker-compose.yaml build

	docker compose -f deploy/users/github/docker-compose.yaml up -d
	docker compose -f deploy/users/gitlab/docker-compose.yaml up -d
	docker compose -f deploy/web-assembly/docker-compose.yaml up -d
	docker compose -f deploy/haproxy/docker-compose.yaml up -d