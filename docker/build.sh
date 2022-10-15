#!/bin/bash

sudo docker network create \
    --driver=bridge \
    --subnet=172.25.0.0/16 \
    --ip-range=172.25.0.0/16 \
    --gateway=172.25.0.1 \
    br0
sudo docker build -t benchmark-image .
