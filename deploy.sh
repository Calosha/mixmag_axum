#!/bin/bash

# deploy.sh
git pull
docker system prune -a -f
docker build -t axum-app .
docker stop mixmag || true
docker rm mixmag || true
docker run -d --name mixmag -p 3000:3000 --restart unless-stopped axum-app
