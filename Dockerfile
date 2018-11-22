FROM rustlang/rust:nightly

WORKDIR /usr/src/app
COPY . .

EXPOSE 8000

RUN cargo install diesel_cli

CMD ["cargo", "run"]
