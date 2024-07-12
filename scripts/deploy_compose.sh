#make sure to add .env, docker-compose.yml, nginx.conf to home

sudo yum update -y
sudo yum install -y docker
sudo systemctl start docker
sudo systemctl enable docker
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
docker-compose --version

sudo usermod -aG docker $USER
newgrp docker
groups $USER
ls -l /var/run/docker.sock

# export CR_PAT=<enter-token-github>
sudo echo $CR_PAT | docker login ghcr.io -u <enter-user-github> --password-stdin

docker system prune
docker rm $(docker ps -a -q)
docker rmi $(docker images -q)
docker volume rm $(docker volume ls -q)

docker pull ghcr.io/crateme-ul/inv-frontend:main
docker pull ghcr.io/crateme-ul/inv-backend:main
docker-compose up --build -d

docker-compose exec inv-db sh -c 'psql -U $POSTGRES_USER $POSTGRES_DB'

# paste this code in
CREATE TABLE Places (
    placeId SERIAL PRIMARY KEY,
    placeName VARCHAR(30) NOT NULL,
    placeType VARCHAR(3) NOT NULL
);

CREATE TABLE Items (
    itemId SERIAL PRIMARY KEY,
    placeId INT REFERENCES Places(placeId) NOT NULL,
    nbOfItems INT not NULL, 
    itemName VARCHAR(30) NOT NULL
);

# add the data as needed