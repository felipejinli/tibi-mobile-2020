#!/bin/bash

sudo docker rm --force tibi_dev_server
sudo docker build -f PostgreDocker . --tag tibi_dev_server
sudo docker run --detach --name tibi_dev_server --publish 5432:5432 tibi_dev_server
