# Character service Dockerfile
#
# Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
FROM debian:12.1-slim

WORKDIR app
COPY target/release/character-service .

CMD ["./character-service"]
