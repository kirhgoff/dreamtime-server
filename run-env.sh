#!/bin/bash -e

# Docker start
docker-compose -f docker/docker-compose.yml up 

# Postgres start
# pg_ctl -D /usr/local/var/postgres -l server.log start

# Postgres stop
# pg_ctl -D /usr/local/var/postgres stop -s -m fast