FROM rust:latest

WORKDIR /demo

COPY . .

RUN cargo install --path .


ENTRYPOINT "cqrs-event-sourcing" -d FOREGROUND