# Dockerfile.backend
FROM rust:latest

WORKDIR /app/backend

COPY ./backend /app/backend

RUN cargo build

CMD ["cargo", "run"]