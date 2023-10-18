FROM rust:1.73.0

WORKDIR usr/app

COPY . .

RUN cargo install cargo-leptos

RUN cargo leptos build --release 