#!/bin/bash

sudo docker rm --force tibi_dev_server_redis
sudo docker build -f RedisDocker . --tag tibi_dev_server_redis
sudo docker run --detach --name tibi_dev_server_redis --publish 6379:6379 tibi_dev_server_redis
