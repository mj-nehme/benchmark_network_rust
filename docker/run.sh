#!/bin/bash

USERNAME="jaafar"
PROJECT="benchmark_network_rust"
IMAGE="benchmark-image"
NIP="172.24.0."
ROLE=$1

if [ "$ROLE" = "server" ]; then
    IP="${NIP}2"
    echo "Running server on ${IP}.."
elif [ "$ROLE" = "client" ]; then
    IP="${NIP}101"
    echo "Running client on ${IP}.."
else
    echo "Please input the role [server|client]"
    exit
fi

sudo docker run \
    --rm \
    -it \
    --cap-drop=all \
    --cap-add net_raw \
    --cap-add net_admin \
    --security-opt no-new-privileges \
    --read-only \
    -u $USERNAME \
    --cpus 4 \
    --memory=1024m \
    --memory-swap=1024m \
    --memory-swappiness=0 \
    --tmpfs /tmp:size=64m \
    --hostname "$PROJECT-$ROLE-$$" \
    --ip $IP \
    --network=br0 \
    --mount type=volume,target=/home/$USERNAME/$PROJECT \
    --mount type=volume,target=/usr/local/cargo/ \
    --security-opt seccomp=seccomp-perf.json \
    $IMAGE
