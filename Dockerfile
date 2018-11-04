FROM rust:latest

WORKDIR /usr/src/myapp
COPY . .

CMD ["cargo", "run"]
