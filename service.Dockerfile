# This is the service Dockerfile
#
# Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
FROM ubuntu:22.04

WORKDIR app
COPY target/release/character-service .

CMD ["./character-service"]
