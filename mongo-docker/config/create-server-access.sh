#!/bin/sh

# Create new user for the server to authenticate
mongosh admin -u ${MONGO_INITDB_ROOT_USERNAME} -p ${MONGO_INITDB_ROOT_PASSWORD} --eval "db.getSiblingDB('${MONGO_SERVERDB_NAME}').createUser({user: '${MONGO_SERVERDB_USERNAME}', pwd: '${MONGO_SERVERDB_PASSWORD}', roles: [ {role: 'readWrite', db: '${MONGO_SERVERDB_NAME}' } ]})"
