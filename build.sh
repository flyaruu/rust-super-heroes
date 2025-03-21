#!/bin/sh
docker build . -t superhero/grpc-locations --target grpc-locations
docker build . -t superhero/rest-heroes --target rest-heroes
docker build . -t superhero/rest-villains --target rest-villains
docker build . -t superhero/rest-fights --target rest-fights
