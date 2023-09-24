# Character service Dockerfile
#
# Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
FROM debian:12.1-slim

RUN apt update && apt install -y openssl

WORKDIR app
COPY target/release/character-service .

CMD ["./character-service"]
