#!/bin/bash

service postgresql start
psql -f init_database.sql
