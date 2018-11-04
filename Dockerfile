FROM rustlang/rust:nightly

WORKDIR /usr/src/app
COPY . .

EXPOSE 8000

CMD ["cargo", "run"]
