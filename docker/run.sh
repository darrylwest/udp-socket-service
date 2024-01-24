#!/bin/sh
# darwest@ebay.com <darryl.west>
# 2024.01.19
#

if docker network ls | fgrep service-net > /dev/null 
then
    echo "ok"
else
    docker network create service-net
fi

export NAME="config-service"

# -v "$PWD":/home/dpw/

# config-data

docker run -it -d -u dpw \
    --network=service-net  \
    --publish 22200:22200/udp  \
    --hostname=config \
    --name $NAME  \
    --mount source=config-data,target=/home/dpw/data \
    "darrylwest/$NAME:latest"

