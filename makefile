build:
	docker build -t inv-backend:local . --force-rm --no-cache

swarm:
	docker swarm init
	docker secret create regcred ~/.docker/config.json
	docker stack deploy -c docker-compose.yml inv --with-registry-auth -d --prune
	docker stack ps inv

