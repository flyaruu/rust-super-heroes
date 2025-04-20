#!/bin/sh
docker build . -t superhero/grpc-locations:latest --target grpc-locations
docker build . -t superhero/rest-heroes:latest --target rest-heroes
docker build . -t superhero/rest-villains:latest --target rest-villains
docker build . -t superhero/rest-fights:latest --target rest-fights
