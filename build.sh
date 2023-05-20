#!/bin/bash
# Dragon Bot Z Character Service's build script
#
# This script is intended to ease the build of this service during development 
# phase.
#
# Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

# Builds project
cargo fmt
cargo clippy
cargo build --release

# Build docker
# Database
from=$(pwd)
mkdir .tmpdocker
cd .tmpdocker

cp ../res/init_database.sql .
cp ../res/init_database.sh .

sudo docker build \
	-f ../database.Dockerfile \
	-t dbz-character-service-db \
	. 

cd $from
rm -r .tmpdocker

# Service
sudo docker build \
	-f service.Dockerfile \
	-t dbz-character-service \
	. \
	--no-cache

# Volume
sudo docker volume create dbz-character-database-volume
